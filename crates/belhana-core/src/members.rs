use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize)]
pub struct FamilyMember {
    pub id: String,
    pub household_id: String,
    pub display_name: String,
    pub age_band: Option<String>,
    pub is_active: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<sqlx::sqlite::SqliteRow> for FamilyMember {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        FamilyMember {
            id: r.get("id"),
            household_id: r.get("household_id"),
            display_name: r.get("display_name"),
            age_band: r.get("age_band"),
            is_active: r.get::<i64, _>("is_active") != 0,
            notes: r.get("notes"),
            created_at: r.get("created_at"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MemberPreference {
    pub id: String,
    pub member_id: String,
    pub kind: String,
    pub ref_type: String,
    pub ref_id: Option<String>,
    pub text_value: Option<String>,
    pub strength: String,
    pub raw_text: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for MemberPreference {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        MemberPreference {
            id: r.get("id"),
            member_id: r.get("member_id"),
            kind: r.get("kind"),
            ref_type: r.get("ref_type"),
            ref_id: r.get("ref_id"),
            text_value: r.get("text_value"),
            strength: r.get("strength"),
            raw_text: r.get("raw_text"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FoodAllergy {
    pub id: String,
    pub member_id: String,
    pub product_id: Option<String>,
    pub allergen: Option<String>,
    pub severity: String,
    pub notes: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for FoodAllergy {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        FoodAllergy {
            id: r.get("id"),
            member_id: r.get("member_id"),
            product_id: r.get("product_id"),
            allergen: r.get("allergen"),
            severity: r.get("severity"),
            notes: r.get("notes"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FoodExclusion {
    pub id: String,
    pub household_id: Option<String>,
    pub member_id: Option<String>,
    pub product_id: String,
    pub reason: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for FoodExclusion {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        FoodExclusion {
            id: r.get("id"),
            household_id: r.get("household_id"),
            member_id: r.get("member_id"),
            product_id: r.get("product_id"),
            reason: r.get("reason"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMember {
    pub display_name: String,
    pub age_band: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePreference {
    pub kind: String,
    pub ref_type: String,
    pub ref_id: Option<String>,
    pub text_value: Option<String>,
    pub strength: Option<String>,
    pub raw_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAllergy {
    pub product_id: Option<String>,
    pub allergen: Option<String>,
    pub severity: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExclusion {
    pub member_id: Option<String>,
    pub product_id: String,
    pub reason: Option<String>,
}

fn validate_pref_kind(kind: &str) -> Result<(), CommonError> {
    const OK: &[&str] = &[
        "like", "dislike", "preferred_ingredient", "preferred_dish", "preferred_method", "dietary",
    ];
    if OK.contains(&kind) {
        Ok(())
    } else {
        Err(CommonError::Validation(format!("invalid preference kind: {kind}")))
    }
}

pub async fn list_members(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<Vec<FamilyMember>, CommonError> {
    let rows = sqlx::query("SELECT * FROM family_members WHERE household_id = ? ORDER BY created_at")
        .bind(household_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(FamilyMember::from).collect())
}

pub async fn create_member(
    pool: &SqlitePool,
    household_id: &str,
    input: &CreateMember,
) -> Result<FamilyMember, CommonError> {
    if input.display_name.trim().is_empty() {
        return Err(CommonError::Validation("member name is required".into()));
    }
    let id = new_id();
    sqlx::query(
        "INSERT INTO family_members (id, household_id, display_name, age_band, notes, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(household_id)
    .bind(input.display_name.trim())
    .bind(&input.age_band)
    .bind(&input.notes)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_member(pool, household_id, &id).await
}

pub async fn get_member(
    pool: &SqlitePool,
    household_id: &str,
    member_id: &str,
) -> Result<FamilyMember, CommonError> {
    let row = sqlx::query("SELECT * FROM family_members WHERE id = ? AND household_id = ?")
        .bind(member_id)
        .bind(household_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("family member".into()))?;
    Ok(FamilyMember::from(row))
}

pub async fn update_member(
    pool: &SqlitePool,
    household_id: &str,
    member_id: &str,
    input: &CreateMember,
) -> Result<FamilyMember, CommonError> {
    get_member(pool, household_id, member_id).await?;
    sqlx::query("UPDATE family_members SET display_name = ?, age_band = ?, notes = ? WHERE id = ?")
        .bind(input.display_name.trim())
        .bind(&input.age_band)
        .bind(&input.notes)
        .bind(member_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    get_member(pool, household_id, member_id).await
}

pub async fn delete_member(
    pool: &SqlitePool,
    household_id: &str,
    member_id: &str,
) -> Result<(), CommonError> {
    get_member(pool, household_id, member_id).await?;
    sqlx::query("DELETE FROM family_members WHERE id = ?")
        .bind(member_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}

pub async fn list_preferences(
    pool: &SqlitePool,
    member_id: &str,
) -> Result<Vec<MemberPreference>, CommonError> {
    let rows = sqlx::query("SELECT * FROM member_preferences WHERE member_id = ? ORDER BY created_at")
        .bind(member_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(MemberPreference::from).collect())
}

pub async fn create_preference(
    pool: &SqlitePool,
    member_id: &str,
    input: &CreatePreference,
) -> Result<MemberPreference, CommonError> {
    validate_pref_kind(&input.kind)?;
    let id = new_id();
    sqlx::query(
        "INSERT INTO member_preferences (id, member_id, kind, ref_type, ref_id, text_value, strength, raw_text, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(member_id)
    .bind(&input.kind)
    .bind(&input.ref_type)
    .bind(&input.ref_id)
    .bind(&input.text_value)
    .bind(input.strength.clone().unwrap_or_else(|| "mild".into()))
    .bind(&input.raw_text)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    let row = sqlx::query("SELECT * FROM member_preferences WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    Ok(MemberPreference::from(row))
}

pub async fn delete_preference(pool: &SqlitePool, member_id: &str, pref_id: &str) -> Result<(), CommonError> {
    sqlx::query("DELETE FROM member_preferences WHERE id = ? AND member_id = ?")
        .bind(pref_id)
        .bind(member_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}

pub async fn list_allergies(pool: &SqlitePool, member_id: &str) -> Result<Vec<FoodAllergy>, CommonError> {
    let rows = sqlx::query("SELECT * FROM food_allergies WHERE member_id = ?")
        .bind(member_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(FoodAllergy::from).collect())
}

pub async fn create_allergy(
    pool: &SqlitePool,
    member_id: &str,
    input: &CreateAllergy,
) -> Result<FoodAllergy, CommonError> {
    if input.product_id.is_none() && input.allergen.is_none() {
        return Err(CommonError::Validation(
            "an allergy must reference a product or an allergen".into(),
        ));
    }
    let id = new_id();
    sqlx::query(
        "INSERT INTO food_allergies (id, member_id, product_id, allergen, severity, notes, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(member_id)
    .bind(&input.product_id)
    .bind(&input.allergen)
    .bind(input.severity.clone().unwrap_or_else(|| "severe".into()))
    .bind(&input.notes)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    let row = sqlx::query("SELECT * FROM food_allergies WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    Ok(FoodAllergy::from(row))
}

pub async fn delete_allergy(pool: &SqlitePool, member_id: &str, allergy_id: &str) -> Result<(), CommonError> {
    sqlx::query("DELETE FROM food_allergies WHERE id = ? AND member_id = ?")
        .bind(allergy_id)
        .bind(member_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}

pub async fn list_exclusions(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<Vec<FoodExclusion>, CommonError> {
    let rows = sqlx::query(
        "SELECT * FROM food_exclusions WHERE household_id = ? OR member_id IN
         (SELECT id FROM family_members WHERE household_id = ?)",
    )
    .bind(household_id)
    .bind(household_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(FoodExclusion::from).collect())
}

pub async fn create_exclusion(
    pool: &SqlitePool,
    household_id: &str,
    input: &CreateExclusion,
) -> Result<FoodExclusion, CommonError> {
    let id = new_id();
    sqlx::query(
        "INSERT INTO food_exclusions (id, household_id, member_id, product_id, reason, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(household_id)
    .bind(&input.member_id)
    .bind(&input.product_id)
    .bind(&input.reason)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    let row = sqlx::query("SELECT * FROM food_exclusions WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    Ok(FoodExclusion::from(row))
}

pub async fn delete_exclusion(pool: &SqlitePool, household_id: &str, excl_id: &str) -> Result<(), CommonError> {
    sqlx::query("DELETE FROM food_exclusions WHERE id = ? AND household_id = ?")
        .bind(excl_id)
        .bind(household_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}
