; ============================================================================
; FocusTrace — Inno Setup 6 installer script
; https://github.com/ShEIH24/FocusTrace
;
; Требования для сборки:
;   • Inno Setup 6.3+  (https://jrsoftware.org/isdl.php)
;   • Собранный бинарь:  ..\target\release\focus-trace.exe
;   • assets\LICENSE.txt
;   • assets\MicrosoftEdgeWebview2Setup.exe  (evergreen bootstrapper от Microsoft)
; ============================================================================

#define AppName      "FocusTrace"
#define AppVersion   "0.1.1.1"
#define AppPublisher "ShEIH24"
#define AppURL       "https://github.com/ShEIH24/FocusTrace"
#define AppExeName   "focus-trace.exe"
#define AppMutexID   "FocusTrace_SingleInstance_Mutex"
#define BinDir       "..\target\release"

; ── Основные параметры пакета ─────────────────────────────────────────────
[Setup]
; GUID однозначно идентифицирует приложение для Windows Installer; не менять.
AppId={{8F3A1B2C-4D5E-6F7A-8B9C-0D1E2F3A4B5C}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}/issues
AppUpdatesURL={#AppURL}/releases
AppCopyright=© 2025 {#AppPublisher}

; Только 64-bit — exe собран как PE32+ (x86-64)
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

; Windows 10 1803+ — минимум для WebView2 Evergreen
MinVersion=10.0.17763

; Устанавливать в папку пользователя — не требует прав администратора
DefaultDirName={localappdata}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

; Блокирует запуск установщика, если приложение уже работает
AppMutex={#AppMutexID}

; Закрыть запущенный процесс перед установкой/обновлением
CloseApplications=yes
CloseApplicationsFilter=*{#AppExeName}*
RestartApplications=no

; Выходной файл
OutputDir=output
OutputBaseFilename=FocusTrace_{#AppVersion}_Setup
Compression=lzma2/ultra64
SolidCompression=yes

; Внешний вид мастера
WizardStyle=modern
WizardSizePercent=120
SetupIconFile=..\src-tauri\icons\icon.ico
UninstallDisplayIcon={app}\{#AppExeName}
UninstallDisplayName={#AppName}

LicenseFile=assets\LICENSE.txt
ShowLanguageDialog=auto
DisableWelcomePage=no
ChangesAssociations=no

; ── Языки ─────────────────────────────────────────────────────────────────
[Languages]
Name: "ru"; MessagesFile: "compiler:Languages\Russian.isl"
Name: "en"; MessagesFile: "compiler:Default.isl"

; ── Переводы строк мастера ────────────────────────────────────────────────
[Messages]

; — Русский —
ru.WelcomeLabel1=Добро пожаловать в мастер установки {#AppName}
ru.WelcomeLabel2={#AppName} — трекер продуктивности с таймером Pomodoro, целями и аналитикой активности. Все данные хранятся локально.%n%nВерсия {#AppVersion} будет установлена на ваш компьютер.%n%nНажмите «Далее», чтобы продолжить.
ru.LicenseLabel=Пожалуйста, прочитайте следующее лицензионное соглашение.
ru.LicenseLabel3=Прокрутите вниз для ознакомления с соглашением. Нажав «Далее», вы принимаете его условия.
ru.LicenseAccepted=Я &принимаю условия соглашения
ru.LicenseNotAccepted=Я &не принимаю условия соглашения
ru.FinishedHeadingLabel=Установка {#AppName} завершена
ru.FinishedLabel={#AppName} {#AppVersion} успешно установлен.%n%nЗапустите приложение из ярлыка на рабочем столе или из меню «Пуск».%n%nДля отслеживания сайтов в браузере установите расширение FocusTrace — инструкция на GitHub.
ru.ClickFinish=Нажмите «Завершить» для выхода из мастера.
ru.SelectDirLabel3=Программа будет установлена в следующую папку.
ru.SelectDirBrowseLabel=Нажмите «Далее», чтобы продолжить, или «Обзор», чтобы выбрать другую папку.
ru.DiskSpaceMBLabel=Требуемый объём свободного места: [mb] МБ.
ru.ReadyLabel1=Мастер готов установить {#AppName} на ваш компьютер.
ru.ReadyLabel2a=Нажмите «Установить» для начала установки или «Назад» для изменения параметров.
ru.StatusInstalling=Установка %1...
ru.StatusRollback=Откат изменений...
ru.UninstallAppFullTitle=Удаление {#AppName}

; — English —
en.WelcomeLabel1=Welcome to the {#AppName} Setup Wizard
en.WelcomeLabel2={#AppName} is a local productivity tracker with a Pomodoro timer, daily goals, and activity analytics. No data ever leaves your device.%n%nVersion {#AppVersion} will be installed on your computer.%n%nClick Next to continue.
en.FinishedHeadingLabel={#AppName} installation complete
en.FinishedLabel={#AppName} {#AppVersion} has been successfully installed.%n%nLaunch the app from the desktop shortcut or Start menu.%n%nTo track browser activity, install the FocusTrace browser extension — see the GitHub README for instructions.
en.ClickFinish=Click Finish to exit the Setup Wizard.

; ── Переводимые строки для Tasks / Run ───────────────────────────────────
[CustomMessages]
ru.StartupEntryDesc=Запускать {#AppName} при старте Windows
ru.StartupEntryGroup=Автозапуск:
ru.WebView2Status=Установка Microsoft WebView2...
ru.LaunchAppDesc=Запустить {#AppName}
ru.ReleasesDesc=Открыть страницу релизов на GitHub

en.StartupEntryDesc=Launch {#AppName} on Windows startup
en.StartupEntryGroup=Autostart:
en.WebView2Status=Installing Microsoft WebView2...
en.LaunchAppDesc=Launch {#AppName}
en.ReleasesDesc=Open releases page on GitHub

; ── Задачи при установке ─────────────────────────────────────────────────
[Tasks]
Name: desktopicon;  Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: startupentry; Description: "{cm:StartupEntryDesc}"; GroupDescription: "{cm:StartupEntryGroup}"; Flags: unchecked

; ── Файлы ─────────────────────────────────────────────────────────────────
[Files]
; Основной исполняемый файл
Source: "{#BinDir}\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion

; WebView2 bootstrapper — копируется во временную папку только если WV2 не установлен
Source: "assets\MicrosoftEdgeWebview2Setup.exe"; DestDir: "{tmp}"; \
  Flags: deleteafterinstall; Check: not IsWebView2Installed

; ── Ярлыки ────────────────────────────────────────────────────────────────
[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\{#AppName}";  Filename: "{app}\{#AppExeName}"; Tasks: desktopicon
Name: "{userstartup}\{#AppName}";  Filename: "{app}\{#AppExeName}"; Tasks: startupentry

; ── Запуск после установки ────────────────────────────────────────────────
[Run]
; 1. Установить WebView2 если отсутствует
Filename: "{tmp}\MicrosoftEdgeWebview2Setup.exe"; \
  Parameters: "/silent /install"; \
  StatusMsg: "{cm:WebView2Status}"; \
  Flags: waituntilterminated; \
  Check: not IsWebView2Installed

; 2. Запустить приложение (опционально)
Filename: "{app}\{#AppExeName}"; \
  Description: "{cm:LaunchAppDesc}"; \
  Flags: nowait postinstall skipifsilent unchecked

; 3. Открыть страницу релизов на GitHub (опционально)
Filename: "{#AppURL}/releases"; \
  Description: "{cm:ReleasesDesc}"; \
  Flags: shellexec postinstall skipifsilent unchecked

; ── Действия при удалении ─────────────────────────────────────────────────
[UninstallRun]
; Завершить процесс перед удалением файлов
Filename: "taskkill.exe"; \
  Parameters: "/f /im {#AppExeName}"; \
  Flags: runhidden skipifdoesntexist; \
  RunOnceId: "TerminateApp"

; ── Удаление оставшихся файлов ────────────────────────────────────────────
[UninstallDelete]
; Удалить всё, что приложение могло создать в папке установки
Type: filesandordirs; Name: "{app}"

; ── Код ──────────────────────────────────────────────────────────────────
[Code]

// Проверяет, установлён ли Microsoft WebView2 (Evergreen).
// Смотрит в HKCU (per-user), HKLM WOW6432Node и HKLM native.
function IsWebView2Installed: Boolean;
var
  Version: String;
begin
  Result :=
    (RegQueryStringValue(HKCU,
        'Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
        'pv', Version)
      and (Version <> '') and (Version <> '0.0.0.0'))
    or
    (RegQueryStringValue(HKLM,
        'SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
        'pv', Version)
      and (Version <> '') and (Version <> '0.0.0.0'))
    or
    (RegQueryStringValue(HKLM,
        'Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
        'pv', Version)
      and (Version <> '') and (Version <> '0.0.0.0'));
end;

// Добавляет ссылку на GitHub в конце страницы «Установка завершена».
procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpFinished then
    WizardForm.FinishedLabel.Caption :=
      WizardForm.FinishedLabel.Caption + #13#10#13#10 +
      'GitHub: {#AppURL}';
end;
