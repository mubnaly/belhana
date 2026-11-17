[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$ErrorActionPreference = 'Stop'
$base = 'http://127.0.0.1:8080'
$loginBody = '{"email":"test@belhana.app","password":"secret123"}'
$reg = Invoke-RestMethod -Method Post -Uri "$base/api/auth/login" -ContentType 'application/json' -Body $loginBody
$tok = $reg.access_token
$H = @{ Authorization = "Bearer $tok" }

$plans = Invoke-RestMethod -Uri "$base/api/meal-plans" -Headers $H
"PLANS: $($plans.Count)"

$p1 = Invoke-RestMethod -Uri "$base/api/meal-plans/$($plans[0].id)" -Headers $H
"PLAN READ: slots=$($p1.slots.Count) first-id=$($p1.slots[0].id)"

Invoke-RestMethod -Method Post -Uri "$base/api/planned-meals/$($p1.slots[0].id)/prepared" -Headers $H -ContentType 'application/json' -Body '{}' | Out-Null
"MARK PREPARED OK"

$rfBody = '{"refresh_token":"' + $reg.refresh_token + '"}'
$rf = Invoke-RestMethod -Method Post -Uri "$base/api/auth/refresh" -ContentType 'application/json' -Body $rfBody
"REFRESH: new=$($rf.access_token.Length -gt 50) refresh=$($rf.refresh_token.Length -gt 20)"
try {
  Invoke-RestMethod -Method Post -Uri "$base/api/auth/refresh" -ContentType 'application/json' -Body $rfBody | Out-Null
  "OLD REFRESH: ACCEPTED (BUG)"
} catch {
  "OLD REFRESH: rejected correctly"
}

$chatBody = '{"messages":[{"role":"user","content":"hi"}]}'
$ai = Invoke-RestMethod -Method Post -Uri "$base/api/ai/chat" -Headers $H -ContentType 'application/json; charset=utf-8' -Body ([System.Text.Encoding]::UTF8.GetBytes($chatBody))
"AI manual-mode: enabled=$($ai.assistant_enabled) reply_len=$($ai.reply.Length)"
