# One-off migration: make every page's local `lang` follow the shared language store.
$root = 'D:\APP\belhana\apps\web\src\routes'
$utf8 = New-Object System.Text.UTF8Encoding($false)

$i18n = 'D:\APP\belhana\apps\web\src\lib\i18n.js'
$s = [System.IO.File]::ReadAllText($i18n, [System.Text.Encoding]::UTF8)
if ($s -notmatch 'writable') {
    $s = $s.Replace('import { browser } from ''$app/environment'';', 'import { browser } from ''$app/environment'';' + "`n" + 'import { writable } from ''svelte/store'';')
}
if ($s -notmatch 'langStore.set\(lang\)') {
    $s = $s.Replace('	listeners.forEach((fn) => fn(lang));', '	listeners.forEach((fn) => fn(lang));' + "`n" + '	langStore.set(lang);')
}
[System.IO.File]::WriteAllText($i18n, $s, $utf8)

$importPattern = 'import \{ ([^}]+) \} from ''\$lib/i18n\.js'''
Get-ChildItem -Recurse $root -Filter *.svelte | Where-Object { $_.Name -ne '+layout.svelte' } | ForEach-Object {
    $text = [System.IO.File]::ReadAllText($_.FullName, [System.Text.Encoding]::UTF8)
    if ($text -notmatch $importPattern) { return }
    if ($text -notmatch 'langStore') {
        $text = [regex]::Replace($text, $importPattern, 'import { ${1}, langStore } from ''$$lib/i18n.js''')
    }
    if ($text -match 'let lang = getLang\(\);' -and $text -notmatch '\$: lang = \$langStore') {
        $text = $text.Replace('let lang = getLang();', 'let lang = getLang();' + "`n" + '	$: lang = $langStore;')
    }
    [System.IO.File]::WriteAllText($_.FullName, $text, $utf8)
    Write-Host "updated $($_.FullName)"
}
