# 检查配置是否改变
# 返回值：0 = 未改变，1 = 已改变

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectPath = Split-Path -Parent $scriptDir
$configPath = Join-Path $projectPath "database\config.json"

try {
    $json = Get-Content $configPath -Raw -Encoding UTF8 | ConvertFrom-Json
    
    if ($json.reminder.is_changed -eq $false) {
        # 配置已改变
        exit 1
    } else {
        # 配置未改变
        exit 0
    }
} catch {
    # 发生错误，假设未改变
    exit 0
}

