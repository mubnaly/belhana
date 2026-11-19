//! Special occasions & family gatherings (العزومات والمناسبات).
//!
//! Menu generation is deterministic and offers three tiers (economical,
//! balanced, elaborate). Portion scaling uses documented assumptions:
//! adult = 1.0 portion, child = recipe.child_portion_factor (default 0.6),
//! both shown transparently and editable by the host.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{catalog, costing, map_db, new_id, now_rfc3339, pricing, recipes};
use belhana_common::units::universal_convert_str;

#[derive(Debug, Serialize)]
pub struct Event {
    pub id: String,
    pub household_id: String,
    pub name: String,
    pub occasion_type: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub adults: i64,
    pub children: i64,
    pub budget_total_minor: Option<i64>,
    pub budget_per_guest_minor: Option<i64>,
    pub meal_type: Option<String>,
    pub style: Option<String>,
    pub include_drinks: bool,
    pub include_sweets: bool,
    pub include_fruit: bool,
    pub include_appetizers: bool,
    pub use_inventory: bool,
    pub status: String,
}

impl From<sqlx::sqlite::SqliteRow> for Event {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        Event {
            id: r.get("id"),
            household_id: r.get("household_id"),
            name: r.get("name"),
            occasion_type: r.get("occasion_type"),
            date: r.get("date"),
            time: r.get("time"),
            adults: r.get("adults"),
            children: r.get("children"),
            budget_total_minor: r.get("budget_total_minor"),
            budget_per_guest_minor: r.get("budget_per_guest_minor"),
            meal_type: r.get("meal_type"),
            style: r.get("style"),
            include_drinks: r.get::<i64, _>("include_drinks") != 0,
            include_sweets: r.get::<i64, _>("include_sweets") != 0,
            include_fruit: r.get::<i64, _>("include_fruit") != 0,
            include_appetizers: r.get::<i64, _>("include_appetizers") != 0,
            use_inventory: r.get::<i64, _>("use_inventory") != 0,
            status: r.get("status"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MenuDish {
    pub id: String,
    pub recipe_id: String,
    pub recipe_name_ar: String,
    pub course: String,
    pub servings: i64,
    pub scale: f64,
    pub cost_minor: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EventMenu {
    pub id: String,
    pub event_id: String,
    pub option_index: i64,
    pub tier: String,
    pub title: String,
    pub total_cost_minor: Option<i64>,
    pub cost_per_guest_minor: Option<i64>,
    pub selected: bool,
    pub dishes: Vec<MenuDish>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub occasion_type: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub adults: Option<i64>,
    pub children: Option<i64>,
    pub budget_total_minor: Option<i64>,
    pub budget_per_guest_minor: Option<i64>,
    pub meal_type: Option<String>,
    pub style: Option<String>,
    pub include_drinks: Option<bool>,
    pub include_sweets: Option<bool>,
    pub include_fruit: Option<bool>,
    pub include_appetizers: Option<bool>,
    pub use_inventory: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEvent {
    pub name: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub adults: Option<i64>,
    pub children: Option<i64>,
    pub budget_total_minor: Option<i64>,
    pub budget_per_guest_minor: Option<i64>,
    pub meal_type: Option<String>,
    pub style: Option<String>,
    pub status: Option<String>,
}

pub const EVENT_STATUSES: &[&str] = &[
    "draft",
    "menu_proposed",
    "menu_approved",
    "shopping",
    "shopping_done",
    "preparing",
    "completed",
];

pub async fn list_events(pool: &SqlitePool, household_id: &str) -> Result<Vec<Event>, CommonError> {
    let rows = sqlx::query("SELECT * FROM events WHERE household_id = ? ORDER BY created_at DESC")
        .bind(household_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(Event::from).collect())
}

pub async fn get_event(pool: &SqlitePool, household_id: &str, id: &str) -> Result<Event, CommonError> {
    let row = sqlx::query("SELECT * FROM events WHERE id = ? AND household_id = ?")
        .bind(id)
        .bind(household_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("event".into()))?;
    Ok(Event::from(row))
}

pub async fn create_event(
    pool: &SqlitePool,
    household_id: &str,
    input: &CreateEvent,
) -> Result<Event, CommonError> {
    if input.name.trim().is_empty() {
        return Err(CommonError::Validation("event name is required".into()));
    }
    let id = new_id();
    sqlx::query(
        "INSERT INTO events (id, household_id, name, occasion_type, date, time, adults, children, budget_total_minor, budget_per_guest_minor,
            meal_type, style, include_drinks, include_sweets, include_fruit, include_appetizers, use_inventory, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'draft', ?, ?)",
    )
    .bind(&id)
    .bind(household_id)
    .bind(input.name.trim())
    .bind(&input.occasion_type)
    .bind(&input.date)
    .bind(&input.time)
    .bind(input.adults.unwrap_or(0).max(0))
    .bind(input.children.unwrap_or(0).max(0))
    .bind(input.budget_total_minor)
    .bind(input.budget_per_guest_minor)
    .bind(&input.meal_type)
    .bind(&input.style)
    .bind(input.include_drinks.unwrap_or(true) as i64)
    .bind(input.include_sweets.unwrap_or(true) as i64)
    .bind(input.include_fruit.unwrap_or(false) as i64)
    .bind(input.include_appetizers.unwrap_or(true) as i64)
    .bind(input.use_inventory.unwrap_or(true) as i64)
    .bind(now_rfc3339())
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_event(pool, household_id, &id).await
}

pub async fn update_event(
    pool: &SqlitePool,
    household_id: &str,
    id: &str,
    input: &UpdateEvent,
) -> Result<Event, CommonError> {
    let _cur = get_event(pool, household_id, id).await?;
    if let Some(s) = &input.status {
        if !EVENT_STATUSES.contains(&s.as_str()) {
            return Err(CommonError::Validation(format!("invalid event status: {s}")));
        }
    }
    sqlx::query(
        "UPDATE events SET name = COALESCE(?, name), date = COALESCE(?, date), time = COALESCE(?, time),
            adults = COALESCE(?, adults), children = COALESCE(?, children),
            budget_total_minor = COALESCE(?, budget_total_minor),
            budget_per_guest_minor = COALESCE(?, budget_per_guest_minor),
            meal_type = COALESCE(?, meal_type), style = COALESCE(?, style),
            status = COALESCE(?, status), updated_at = ?
         WHERE id = ?",
    )
    .bind(&input.name)
    .bind(&input.date)
    .bind(&input.time)
    .bind(input.adults)
    .bind(input.children)
    .bind(input.budget_total_minor)
    .bind(input.budget_per_guest_minor)
    .bind(&input.meal_type)
    .bind(&input.style)
    .bind(&input.status)
    .bind(now_rfc3339())
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_event(pool, household_id, id).await
}

/// Effective portion equivalents for scaling: adults count as 1.0, children
/// as the recipe's child factor. Documented assumption, not a hidden conversion.
pub fn portion_equivalents(adults: i64, children: i64, child_factor: f64) -> f64 {
    adults.max(0) as f64 + (children.max(0) as f64 * child_factor.clamp(0.3, 0.9))
}

struct DishPick {
    recipe: recipes::Recipe,
    course: &'static str,
}

/// Generates three deterministic menu options for the event.
pub async fn generate_menus(
    pool: &SqlitePool,
    household_id: &str,
    event_id: &str,
) -> Result<Vec<EventMenu>, CommonError> {
    let event = get_event(pool, household_id, event_id).await?;
    // clear previous proposals
    sqlx::query("DELETE FROM event_menus WHERE event_id = ?")
        .bind(event_id)
        .execute(pool)
        .await
        .map_err(map_db)?;

    let meal_type = event.meal_type.clone().unwrap_or_else(|| "lunch".into());
    let diet = crate::households::selected_diet_id(pool, household_id).await?;
    let all = recipes::list_recipes(pool, household_id, Some(&meal_type), None, diet.as_deref(), 200, 0).await?;
    if all.is_empty() {
        return Err(CommonError::Validation(format!(
            "لا توجد وصفات مناسبة لنوع الوجبة {meal_type} — أضف وصفات أولا"
        )));
    }
    // cost every recipe per serving for tier selection
    let hh = crate::households::get_household(pool, household_id).await?;
    let mut costed: Vec<(recipes::Recipe, i64)> = Vec::new();
    for r in &all {
        let c = costing::compute_recipe_cost(pool, &r.id, r.default_servings, &hh.price_policy).await?;
        let per_serving = c
            .per_serving_minor
            .unwrap_or(c.known_total_minor / r.default_servings.max(1) as i64);
        costed.push((r.clone(), per_serving));
    }
    let mut sorted = costed.clone();
    sorted.sort_by(|a, b| a.1.cmp(&b.1)); // cheapest first

    let child_factor = all[0].child_portion_factor;
    let guests = portion_equivalents(event.adults, event.children, child_factor);
    let course_sets: [(Vec<DishPick>, &str); 3] = [
        (
            vec![
                DishPick { recipe: all[0].clone(), course: "main" },
            ],
            "economical",
        ),
        (build_balanced(&sorted, &event, meal_type.as_str()), "balanced"),
        (build_elaborate(&sorted, &event, meal_type.as_str()), "elaborate"),
    ];

    let mut menus = Vec::new();
    for (idx, (dishes, tier)) in course_sets.iter().enumerate() {
        if dishes.is_empty() {
            continue;
        }
        let menu_id = new_id();
        sqlx::query(
            "INSERT INTO event_menus (id, event_id, option_index, tier, title, selected) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&menu_id)
        .bind(event_id)
        .bind(idx as i64)
        .bind(tier)
        .bind(match *tier {
            "economical" => "القائمة الاقتصادية",
            "balanced" => "القائمة المتوازنة",
            _ => "القائمة المناسبة للمناسبات",
        })
        .bind(if idx == 0 { 1 } else { 0 })
        .execute(pool)
        .await
        .map_err(map_db)?;
        let mut total = 0i64;
        let mut complete = true;
        for d in dishes {
            let servings = guests.ceil() as i64;
            let scale = if d.recipe.default_servings > 0 {
                guests / d.recipe.default_servings as f64
            } else {
                1.0
            };
            let cost = costing::compute_recipe_cost(pool, &d.recipe.id, servings, &hh.price_policy).await?;
            if cost.unknown_count > 0 {
                complete = false;
            }
            let c = cost.total_minor.unwrap_or(cost.known_total_minor);
            total += c;
            sqlx::query(
                "INSERT INTO event_menu_dishes (id, menu_id, recipe_id, course, servings, scale, cost_minor) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(new_id())
            .bind(&menu_id)
            .bind(&d.recipe.id)
            .bind(d.course)
            .bind(servings)
            .bind(scale)
            .bind(c)
            .execute(pool)
            .await
            .map_err(map_db)?;
        }
        let guest_count = (event.adults + event.children).max(1);
        let per_guest = belhana_common::Money::egp(total).scale_ratio(1, guest_count).minor;
        sqlx::query("UPDATE event_menus SET total_cost_minor = ?, cost_per_guest_minor = ? WHERE id = ?")
            .bind(total)
            .bind(per_guest)
            .bind(&menu_id)
            .execute(pool)
            .await
            .map_err(map_db)?;
        let _ = complete;
        menus.push(get_menu(pool, &menu_id).await?);
    }
    sqlx::query("UPDATE events SET status = 'menu_proposed', updated_at = ? WHERE id = ?")
        .bind(now_rfc3339())
        .bind(event_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(menus)
}

fn pick_by_tier(
    sorted: &[(recipes::Recipe, i64)],
    start: usize,
    filter: impl Fn(&recipes::Recipe) -> bool,
) -> Option<&recipes::Recipe> {
    sorted
        .iter()
        .skip(start)
        .find(|(r, _)| filter(r))
        .map(|(r, _)| r)
}

fn build_balanced(
    sorted: &[(recipes::Recipe, i64)],
    event: &Event,
    meal_type: &str,
) -> Vec<DishPick> {
    let mut out = Vec::new();
    if let Some(r) = sorted.iter().skip(1).find(|(r, _)| r.meal_types.iter().any(|m| m == meal_type)) {
        out.push(DishPick { recipe: r.0.clone(), course: "main" });
    }
    if let Some(r) = pick_by_tier(sorted, 0, |r| r.meal_types.iter().any(|m| m == "lunch")) {
        if out.iter().all(|d| d.recipe.id != r.id) {
            out.push(DishPick { recipe: r.clone(), course: "side" });
        }
    }
    if event.include_appetizers {
        if let Some(r) = pick_by_tier(sorted, 0, |r| r.meal_types.iter().any(|m| m == "breakfast")) {
            if out.iter().all(|d| d.recipe.id != r.id) {
                out.push(DishPick { recipe: r.clone(), course: "appetizer" });
            }
        }
    }
    if event.include_sweets {
        if let Some(r) = pick_by_tier(sorted, 0, |r| r.meal_types.iter().any(|m| m == "dessert")) {
            out.push(DishPick { recipe: r.clone(), course: "dessert" });
        }
    }
    out
}

fn build_elaborate(
    sorted: &[(recipes::Recipe, i64)],
    event: &Event,
    meal_type: &str,
) -> Vec<DishPick> {
    let mut out = Vec::new();
    // premium main = most expensive suitable
    if let Some((r, _)) = sorted.iter().rev().find(|(r, _)| r.meal_types.iter().any(|m| m == meal_type)) {
        out.push(DishPick { recipe: r.clone(), course: "main" });
    }
    if let Some((r, _)) = sorted.iter().rev().find(|(r, _)| r.meal_types.iter().any(|m| m == "lunch") && out.iter().all(|d| d.recipe.id != r.id)) {
        out.push(DishPick { recipe: r.clone(), course: "side" });
    }
    if let Some((r, _)) = sorted.iter().rev().find(|(r, _)| r.meal_types.iter().any(|m| m == "breakfast") && out.iter().all(|d| d.recipe.id != r.id)) {
        out.push(DishPick { recipe: r.clone(), course: "appetizer" });
    }
    if event.include_sweets {
        if let Some((r, _)) = sorted.iter().rev().find(|(r, _)| r.meal_types.iter().any(|m| m == "dessert")) {
            out.push(DishPick { recipe: r.clone(), course: "dessert" });
        }
    }
    out
}

pub async fn get_menu(pool: &SqlitePool, menu_id: &str) -> Result<EventMenu, CommonError> {
    let row = sqlx::query("SELECT * FROM event_menus WHERE id = ?")
        .bind(menu_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("event menu".into()))?;
    let menu = EventMenu {
        id: row.get("id"),
        event_id: row.get("event_id"),
        option_index: row.get("option_index"),
        tier: row.get("tier"),
        title: row.get("title"),
        total_cost_minor: row.get("total_cost_minor"),
        cost_per_guest_minor: row.get("cost_per_guest_minor"),
        selected: row.get::<i64, _>("selected") != 0,
        dishes: Vec::new(),
    };
    let dishes = sqlx::query(
        "SELECT d.*, r.name_ar AS recipe_name FROM event_menu_dishes d
         JOIN recipes r ON r.id = d.recipe_id WHERE d.menu_id = ?",
    )
    .bind(menu_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let mut m = menu;
    m.dishes = dishes
        .into_iter()
        .map(|r| MenuDish {
            id: r.get("id"),
            recipe_id: r.get("recipe_id"),
            recipe_name_ar: r.get("recipe_name"),
            course: r.get("course"),
            servings: r.get("servings"),
            scale: r.get("scale"),
            cost_minor: r.get("cost_minor"),
        })
        .collect();
    Ok(m)
}

pub async fn list_menus(pool: &SqlitePool, event_id: &str) -> Result<Vec<EventMenu>, CommonError> {
    let ids = sqlx::query("SELECT id FROM event_menus WHERE event_id = ? ORDER BY option_index")
        .bind(event_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    let mut out = Vec::new();
    for r in ids {
        out.push(get_menu(pool, &r.get::<String, _>("id")).await?);
    }
    Ok(out)
}

/// Approves one menu option for the event.
pub async fn approve_menu(
    pool: &SqlitePool,
    household_id: &str,
    event_id: &str,
    menu_id: &str,
) -> Result<EventMenu, CommonError> {
    get_event(pool, household_id, event_id).await?;
    let menu = get_menu(pool, menu_id).await?;
    if menu.event_id != event_id {
        return Err(CommonError::Validation("menu does not belong to this event".into()));
    }
    sqlx::query("UPDATE event_menus SET selected = 0 WHERE event_id = ?")
        .bind(event_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    sqlx::query("UPDATE event_menus SET selected = 1 WHERE id = ?")
        .bind(menu_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    sqlx::query("UPDATE events SET status = 'menu_approved', updated_at = ? WHERE id = ?")
        .bind(now_rfc3339())
        .bind(event_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    get_menu(pool, menu_id).await
}

/// Scaled ingredient quantities across the selected menu, with per-guest cost
/// and shopping list. Uses selected price observations; unpriced items are
/// flagged, never guessed.
pub async fn generate_event_shopping(
    pool: &SqlitePool,
    household_id: &str,
    event_id: &str,
) -> Result<serde_json::Value, CommonError> {
    let event = get_event(pool, household_id, event_id).await?;
    let selected = sqlx::query("SELECT id FROM event_menus WHERE event_id = ? AND selected = 1")
        .bind(event_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::Validation("no approved menu for this event".into()))?;
    let menu = get_menu(pool, &selected.get::<String, _>("id")).await?;
    let hh = crate::households::get_household(pool, household_id).await?;

    sqlx::query("DELETE FROM event_shopping_items WHERE event_id = ?")
        .bind(event_id)
        .execute(pool)
        .await
        .map_err(map_db)?;

    let mut required: std::collections::HashMap<String, (f64, String)> = Default::default();
    for d in &menu.dishes {
        let ings = costing::ingredient_quantities(pool, &d.recipe_id, d.servings).await?;
        for (pid, qty, unit) in ings {
            let e = required.entry(pid.clone()).or_insert((0.0, unit.clone()));
            if e.1 == unit {
                e.0 += qty;
            } else if let Some(c) = universal_convert_str(qty, &unit, &e.1) {
                e.0 += c;
            } else {
                let key = format!("{pid}|{unit}");
                required.insert(key, (qty, unit.clone()));
            }
        }
    }

    let mut estimated_total = 0i64;
    let mut unpriced = Vec::new();
    let guest_count = (event.adults + event.children).max(1);
    for (pid, (qty, unit)) in &required {
        let product = catalog::get_product(pool, pid).await?;
        let obs = pricing::best_observation_for_product(pool, pid, &hh.price_policy).await?;
        let per = obs.as_ref().and_then(|o| costing::price_per_unit(o, unit));
        let total = per.map(|p| (p * qty).round() as i64);
        if let Some(t) = total {
            estimated_total += t;
        } else {
            unpriced.push(product.name_ar.clone());
        }
        sqlx::query(
            "INSERT INTO event_shopping_items (id, event_id, product_id, quantity, unit, est_price_minor, est_total_minor, price_observation_id)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(event_id)
        .bind(pid)
        .bind(*qty)
        .bind(unit)
        .bind(per.map(|p| p.round() as i64))
        .bind(total)
        .bind(obs.as_ref().map(|o| o.id.clone()))
        .execute(pool)
        .await
        .map_err(map_db)?;
    }
    sqlx::query("UPDATE events SET status = 'shopping', updated_at = ? WHERE id = ?")
        .bind(now_rfc3339())
        .bind(event_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    let items = sqlx::query(
        "SELECT i.*, p.name_ar AS name FROM event_shopping_items i JOIN products p ON p.id = i.product_id WHERE i.event_id = ?",
    )
    .bind(event_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let per_guest = belhana_common::Money::egp(estimated_total).scale_ratio(1, guest_count).minor;
    Ok(serde_json::json!({
        "event_id": event_id,
        "menu": menu,
        "guests": guest_count,
        "portion_equivalents": portion_equivalents(event.adults, event.children, menu.dishes.first().map(|_d| 0.6).unwrap_or(0.6)),
        "estimated_total_minor": estimated_total,
        "cost_per_guest_minor": per_guest,
        "items": items.into_iter().map(|r| serde_json::json!({
            "product_id": r.get::<String, _>("product_id"),
            "name_ar": r.get::<String, _>("name"),
            "quantity": r.get::<f64, _>("quantity"),
            "unit": r.get::<String, _>("unit"),
            "est_total_minor": r.get::<Option<i64>, _>("est_total_minor"),
        })).collect::<Vec<_>>(),
        "unpriced_products": unpriced,
    }))
}

/// Records an actual occasion cost and marks the event lifecycle.
pub async fn record_actual_cost(
    pool: &SqlitePool,
    household_id: &str,
    event_id: &str,
    category: &str,
    amount_minor: i64,
    note: Option<&str>,
) -> Result<Event, CommonError> {
    get_event(pool, household_id, event_id).await?;
    sqlx::query(
        "INSERT INTO event_actual_costs (id, event_id, category, amount_minor, note, date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(event_id)
    .bind(category)
    .bind(amount_minor)
    .bind(note)
    .bind(crate::today().format("%Y-%m-%d").to_string())
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    // also a household expense for budget tracking
    sqlx::query(
        "INSERT INTO actual_expenses (id, household_id, category, amount_minor, date, event_id, note, created_at)
         VALUES (?, ?, 'occasion', ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(amount_minor)
    .bind(crate::today().format("%Y-%m-%d").to_string())
    .bind(event_id)
    .bind(note)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    update_event(
        pool,
        household_id,
        event_id,
        &UpdateEvent {
            name: None,
            date: None,
            time: None,
            adults: None,
            children: None,
            budget_total_minor: None,
            budget_per_guest_minor: None,
            meal_type: None,
            style: None,
            status: Some("completed".into()),
        },
    )
    .await
}

/// Planned vs actual comparison.
pub async fn planned_vs_actual(
    pool: &SqlitePool,
    household_id: &str,
    event_id: &str,
) -> Result<serde_json::Value, CommonError> {
    get_event(pool, household_id, event_id).await?;
    let planned: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(est_total_minor), 0) FROM event_shopping_items WHERE event_id = ?",
    )
    .bind(event_id)
    .fetch_one(pool)
    .await
    .map_err(map_db)?;
    let actual: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount_minor), 0) FROM event_actual_costs WHERE event_id = ?",
    )
    .bind(event_id)
    .fetch_one(pool)
    .await
    .map_err(map_db)?;
    Ok(serde_json::json!({
        "planned_minor": planned,
        "actual_minor": actual,
        "difference_minor": planned - actual,
    }))
}
