# Smoke-test the endpoints wired by the final web screens.
$ErrorActionPreference = 'Stop'
$base = 'http://127.0.0.1:8080'

$health = Invoke-RestMethod "$base/healthz"
Write-Host "health: $($health.app) ok=$($health.ok)"

$login = @{ email = 'test@belhana.app'; password = 'secret123' } | ConvertTo-Json
$reg = Invoke-RestMethod "$base/api/auth/login" -Method Post -Body $login -ContentType 'application/json'
$tok = $reg.access_token
$headers = @{ Authorization = "Bearer $tok" }

$hh = Invoke-RestMethod "$base/api/household" -Headers $headers
Write-Host "household: $($hh.name) size=$($hh.household_size) policy=$($hh.price_policy)"
# Send only ASCII fields: PowerShell mangles Arabic response strings when re-encoding.
$updated = Invoke-RestMethod "$base/api/household" -Method Put -Headers $headers -ContentType 'application/json' -Body (@{
    household_size = $hh.household_size
    weekly_budget_minor = $hh.weekly_budget_minor; monthly_budget_minor = $hh.monthly_budget_minor
    price_policy = $hh.price_policy
} | ConvertTo-Json)
Write-Host "household PUT ok: $($updated.id -eq $hh.id)"

$batches = Invoke-RestMethod "$base/api/imports/batches" -Headers $headers
$batch = $batches[0]
$source = Invoke-RestMethod "$base/api/imports/batches/$($batch.id)/source-post" -Headers $headers
Write-Host "batch: $($batch.batch_status); source raw length=$($source.raw_text.Length); date status=$($source.date_status)"
$observations = Invoke-RestMethod "$base/api/imports/batches/$($batch.id)/observations" -Headers $headers
Write-Host "observations: $($observations.Count); first status=$($observations[0].review_status)"

$preview = Invoke-RestMethod "$base/api/imports/parse" -Method Post -Headers $headers -ContentType 'application/json' -Body (Get-Content "$PSScriptRoot\parse_body.json" -Raw -Encoding UTF8)
Write-Host "preview date=$($preview.proposed_date) lines=$($preview.lines.Count)"
