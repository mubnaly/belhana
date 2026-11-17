[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$ErrorActionPreference = 'Stop'
$base = 'http://127.0.0.1:8080'
$reg = Invoke-RestMethod -Method Post -Uri "$base/api/auth/login" -ContentType 'application/json' -Body '{"email":"test@belhana.app","password":"secret123"}'
$H = @{ Authorization = "Bearer $($reg.access_token)" }

# event flow
$evBody = [System.Text.Encoding]::UTF8.GetBytes('{"name":"eid_visit","adults":6,"children":2,"occasion_type":"family_visit"}')
$ev = Invoke-RestMethod -Method Post -Uri "$base/api/events" -Headers $H -ContentType 'application/json; charset=utf-8' -Body $evBody
$menus = Invoke-RestMethod -Method Post -Uri "$base/api/events/$($ev.id)/menus" -Headers $H -ContentType 'application/json' -Body '{}'
"MENUS: $($menus.Count) tiers=$(( $menus | ForEach-Object { $_.tier }) -join ',')"
Invoke-RestMethod -Method Post -Uri "$base/api/events/$($ev.id)/menus/approve/$($menus[0].id)" -Headers $H -ContentType 'application/json' -Body '{}' | Out-Null
"MENU APPROVED: $($menus[0].tier)"
$shop = Invoke-RestMethod -Method Post -Uri "$base/api/events/$($ev.id)/shopping" -Headers $H -ContentType 'application/json' -Body '{}'
"EVENT SHOPPING: items=$($shop.items.Count) est=$($shop.estimated_total_minor)"
$costBody = [System.Text.Encoding]::UTF8.GetBytes('{"category":"meat","amount_minor":15000,"note":"لحم"}')
Invoke-RestMethod -Method Post -Uri "$base/api/events/$($ev.id)/actual-costs" -Headers $H -ContentType 'application/json; charset=utf-8' -Body $costBody | Out-Null
$pva = Invoke-RestMethod -Uri "$base/api/events/$($ev.id)/planned-vs-actual" -Headers $H
"PLANNED-VS-ACTUAL: planned=$($pva.planned_minor) actual=$($pva.actual_minor)"

# recipe cost
$recipes = Invoke-RestMethod -Uri "$base/api/recipes?limit=3" -Headers $H
$cost = Invoke-RestMethod -Uri "$base/api/recipes/$($recipes[0].id)/cost?servings=4" -Headers $H
"RECIPE COST: name=$($recipes[0].name_ar) total=$($cost.total_minor) per_serving=$($cost.per_serving_minor) unknown=$($cost.unknown_count)"

# batch guides
$guides = Invoke-RestMethod -Uri "$base/api/batch-guides" -Headers $H
"BATCH GUIDES: $($guides.Count)"

# merchant list
$merchants = Invoke-RestMethod -Uri "$base/api/merchants" -Headers $H
"MERCHANTS: $($merchants.Count) first=$($merchants[0].name)"

# imports parse preview
$parseBody = [System.Text.Encoding]::UTF8.GetBytes('{"raw_text":"أسعار اليوم الجمعة 20/11/2026\nطماطم 30\nبصل 25 نص كيلو"}')
$parse = Invoke-RestMethod -Method Post -Uri "$base/api/imports/parse" -Headers $H -ContentType 'application/json; charset=utf-8' -Body $parseBody
"PARSE: proposed_date=$($parse.proposed_date) lines=$($parse.lines.Count) weekday=$($parse.weekday_word)"
