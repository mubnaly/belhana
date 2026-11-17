[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$base = 'http://127.0.0.1:8080'
$reg = Invoke-RestMethod -Method Post -Uri "$base/api/auth/login" -ContentType 'application/json' -Body '{"email":"test@belhana.app","password":"secret123"}'
$H = @{ Authorization = "Bearer $($reg.access_token)" }
$parse = Invoke-RestMethod -Method Post -Uri "$base/api/imports/parse" -Headers $H -ContentType 'application/json; charset=utf-8' -InFile 'D:\APP\belhana\scripts\parse_body.json'
"proposed_date=$($parse.proposed_date) weekday=$($parse.weekday_word) mismatch=$($parse.weekday_mismatch) lines=$($parse.lines.Count)"
"original_date_text=$($parse.original_date_text)"
foreach ($l in $parse.lines) { "LINE: label=$($l.label) price=$($l.prices_minor -join '/') unit=$($l.proposed_unit) flags=$($l.flags -join ',')" }
