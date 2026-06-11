# OLED Wallpaper Automator - Windows Task Scheduler Setup

$ProjectDir = if ($PSScriptRoot) { $PSScriptRoot } else { Get-Location }
$ExeFile = Join-Path $ProjectDir "oled-wallpaper-automator.exe"

# Ensure the executable exists, downloading it if missing
if (-not (Test-Path $ExeFile)) {
    Write-Host "oled-wallpaper-automator.exe not found!"
    Write-Host "Downloading precompiled binary from GitHub..."
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        $DownloadUrl = "https://github.com/scuba3198/oled-wallpaper-automator/releases/latest/download/oled-wallpaper-automator.exe"
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExeFile
        Write-Host "✅ Successfully downloaded binary."
    } catch {
        Write-Error "Failed to download binary: $_"
        Write-Host "`n[ERROR] Could not download oled-wallpaper-automator.exe automatically."
        Write-Host "Please check your internet connection or download it manually from:"
        Write-Host "https://github.com/scuba3198/oled-wallpaper-automator/releases"
        Write-Host "Place it in: $ProjectDir"
        Exit
    }
}

# Action: Run oled-wallpaper-automator.exe directly
$Action = New-ScheduledTaskAction -Execute $ExeFile -WorkingDirectory $ProjectDir

# Trigger: Every day at 9:00 AM
$Trigger = New-ScheduledTaskTrigger -Daily -At 9:00AM

# Settings: Allow running even if on battery, wake computer to run, etc.
# -StartWhenAvailable ensures it runs as soon as you turn your PC on if the 9 AM slot was missed.
# -RunOnlyIfNetworkAvailable prevents failure if Wi-Fi isn't ready.
$Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -WakeToRun -StartWhenAvailable -RunOnlyIfNetworkAvailable

# Register the task
$TaskName = "DailyOLEDWallpaper"
Register-ScheduledTask -Action $Action -Trigger $Trigger -Settings $Settings -TaskName $TaskName -Description "Daily OLED wallpaper updater from 4kwallpapers.com" -Force

Write-Host "`n✅ Task '$TaskName' has been scheduled successfully!"
Write-Host "It will run every day at 9:00 AM."
Write-Host "You can manually run it from 'Task Scheduler' to test."

