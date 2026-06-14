# OLED Wallpaper Automator - Unistall Script

$TaskName = "DailyOLEDWallpaper"

# Remove the scheduled task
if (Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue) {
    Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
    Write-Host "✅ Scheduled task '$TaskName' has been successfully removed."
} else {
    Write-Host "ℹ️ Task '$TaskName' was not found (it may have already been removed)."
}

Write-Host "`nTo finish uninstallation:"
Write-Host "1. You can now safely delete this entire folder: $(Get-Location)"
Write-Host "2. Your current wallpaper will remain as-is."

Write-Host "`nPress any key to close..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

