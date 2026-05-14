; ============================================================================
; FocusTrace – скрипт установщика Inno Setup 6
; https://github.com/ShEIH24/FocusTrace
; ============================================================================

#define AppName      "FocusTrace"
#define AppVersion   "0.1.0"
#define AppPublisher "ShEIH24"
#define AppURL       "https://github.com/ShEIH24/FocusTrace"
#define AppExeName   "focus-trace.exe"
#define BinDir       "..\target\release"
#define ReadmeURL    "https://github.com/ShEIH24/FocusTrace/blob/main/README.md"

[Setup]
AppId={{8F3A1B2C-4D5E-6F7A-8B9C-0D1E2F3A4B5C}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}/issues
AppUpdatesURL={#AppURL}/releases

DefaultDirName={localappdata}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

OutputDir=output
OutputBaseFilename=FocusTrace_{#AppVersion}_Setup
Compression=lzma2/ultra64
SolidCompression=yes
InternalCompressLevel=ultra64

WizardStyle=modern
WizardSizePercent=120
SetupIconFile=..\src-tauri\icons\icon.ico
UninstallDisplayIcon={app}\{#AppExeName}
UninstallDisplayName={#AppName}

MinVersion=10.0.17763

CloseApplications=yes
CloseApplicationsFilter=*{#AppExeName}
RestartApplications=yes

ShowLanguageDialog=no
ChangesAssociations=no
DisableWelcomePage=no

LicenseFile=assets\LICENSE.txt

[Languages]
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[Messages]
WelcomeLabel1=Добро пожаловать в мастер установки {#AppName}
WelcomeLabel2=Этот мастер установит {#AppName} {#AppVersion} на ваш компьютер.%n%n{#AppName} — трекер продуктивности, который отслеживает активность локально. Никакие данные не покидают ваше устройство.%n%nНажмите «Далее», чтобы продолжить.
LicenseLabel=Пожалуйста, внимательно прочитайте следующее лицензионное соглашение.
LicenseLabel3=Прокрутите вниз, чтобы ознакомиться со всем соглашением. Чтобы продолжить установку, вы должны принять условия соглашения.
LicenseAccepted=Я &принимаю условия соглашения
LicenseNotAccepted=Я &не принимаю условия соглашения
FinishedHeadingLabel=Установка {#AppName} завершена
FinishedLabel=Установка {#AppName} {#AppVersion} успешно завершена.%n%nЗапустите приложение, выбрав соответствующий значок.%n%nСовет: установите браузерное расширение, чтобы отслеживать посещаемые сайты — инструкция в README на GitHub.
ClickFinish=Нажмите «Завершить» для выхода из мастера установки.
SelectDirLabel3=Программа будет установлена в следующую папку.
SelectDirBrowseLabel=Нажмите «Далее», чтобы продолжить. Если вы хотите выбрать другую папку, нажмите «Обзор».
DiskSpaceMBLabel=Требуемый объём свободного места на диске: [mb] МБ.
ReadyLabel1=Мастер готов установить {#AppName} на ваш компьютер.
ReadyLabel2a=Нажмите «Установить» для начала установки или «Назад» для изменения параметров.
StatusInstalling=Установка %1...
StatusRollback=Откат изменений...
UninstallAppFullTitle=Удаление {#AppName}

[Tasks]
Name: desktopicon;  Description: "Создать &ярлык на рабочем столе";           GroupDescription: "Дополнительные значки:"; Flags: unchecked
Name: startupentry; Description: "Запускать {#AppName} при старте Windows";   GroupDescription: "Автозапуск:";            Flags: unchecked

[Files]
Source: "{#BinDir}\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "assets\MicrosoftEdgeWebview2Setup.exe"; DestDir: "{tmp}"; Flags: deleteafterinstall; Check: not IsWebView2Installed

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}_is1"; ValueType: string; ValueName: "DisplayVersion"; ValueData: "{#AppVersion}"; Flags: uninsdeletevalue

[Icons]
Name: "{autoprograms}\{#AppName}";  Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\{#AppName}";   Filename: "{app}\{#AppExeName}"; Tasks: desktopicon
Name: "{userstartup}\{#AppName}";   Filename: "{app}\{#AppExeName}"; Tasks: startupentry

[Run]
; Установка WebView2 если отсутствует
Filename: "{tmp}\MicrosoftEdgeWebview2Setup.exe"; Parameters: "/silent /install"; StatusMsg: "Установка Microsoft WebView2..."; Flags: waituntilterminated; Check: not IsWebView2Installed

; Запуск приложения после установки
Filename: "{app}\{#AppExeName}"; Description: "Запустить {#AppName}"; Flags: nowait postinstall skipifsilent unchecked

; Открытие README на GitHub в браузере (опционально)
Filename: "{#ReadmeURL}"; Description: "Открыть README на GitHub (инструкция по расширению и настройке)"; Flags: shellexec postinstall skipifsilent unchecked

[Code]

function IsWebView2Installed: Boolean;
var
  version: String;
begin
  Result :=
    RegQueryStringValue(HKCU,
      'Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
      'pv', version) and (version <> '') and (version <> '0.0.0.0')
    or
    RegQueryStringValue(HKLM,
      'SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
      'pv', version) and (version <> '') and (version <> '0.0.0.0');
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpFinished then
  begin
    WizardForm.FinishedLabel.Caption :=
      WizardForm.FinishedLabel.Caption + #13#10#13#10 +
      'Исходный код, документация и поддержка:' + #13#10 +
      '{#AppURL}';
  end;
end;

function NextButtonClick(CurPageID: Integer): Boolean;
begin
  Result := True;
end;
