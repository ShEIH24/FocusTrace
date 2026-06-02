; ============================================================================
; FocusTrace — Inno Setup 6 installer script
; https://github.com/ShEIH24/FocusTrace
;
; Требования для сборки:
;   • Inno Setup 6.3+  (https://jrsoftware.org/isdl.php)
;   • Собранный бинарь:  ..\target\release\focus-trace.exe
;   • assets\LICENSE.txt
;   • assets\MicrosoftEdgeWebview2Setup.exe  (evergreen bootstrapper от Microsoft)
;   • ..\extensions\focustrace\*             (исходники расширения браузера)
; ============================================================================

#define AppName      "FocusTrace"
#define AppVersion   "0.1.3"
#define AppPublisher "ShEIH24"
#define AppURL       "https://github.com/ShEIH24/FocusTrace"
#define AppExeName   "focus-trace.exe"
#define AppMutexID   "FocusTrace_SingleInstance_Mutex"
#define BinDir       "..\target\release"

; ── Основные параметры пакета ─────────────────────────────────────────────
[Setup]
AppId={{8F3A1B2C-4D5E-6F7A-8B9C-0D1E2F3A4B5C}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}/issues
AppUpdatesURL={#AppURL}/releases
AppCopyright=© 2025 {#AppPublisher}

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

AppMutex={#AppMutexID}
CloseApplications=yes
CloseApplicationsFilter=*{#AppExeName}*
RestartApplications=no

OutputDir=output
OutputBaseFilename=FocusTrace_{#AppVersion}_Setup
Compression=lzma2/ultra64
SolidCompression=yes

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
ru.FinishedLabel={#AppName} {#AppVersion} успешно установлен.%n%nЗапустите приложение из ярлыка на рабочем столе или из меню «Пуск».
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
en.FinishedLabel={#AppName} {#AppVersion} has been successfully installed.%n%nLaunch the app from the desktop shortcut or Start menu.
en.ClickFinish=Click Finish to exit the Setup Wizard.

; ── Пользовательские строки ───────────────────────────────────────────────
[CustomMessages]

; — Русский —
ru.StartupEntryDesc=Запускать {#AppName} при старте Windows
ru.StartupEntryGroup=Автозапуск:
ru.WebView2Status=Установка Microsoft WebView2...
ru.LaunchAppDesc=Запустить {#AppName}
ru.ReleasesDesc=Открыть страницу релизов на GitHub

ru.ExtPageTitle=Расширение для браузера
ru.ExtPageDesc=Отслеживание посещённых сайтов
ru.ExtIntroText=Расширение FocusTrace отправляет данные об активной вкладке прямо в приложение (всё локально, ничего не уходит в интернет).%nВыберите браузеры, в которых хотите настроить расширение:
ru.ExtNoBrowsers=Поддерживаемые браузеры не найдены. После установки вы сможете настроить расширение вручную из папки extension\ в директории приложения.
ru.ExtStepsChrome=Инструкция (Chrome / Edge / Opera / Яндекс / Brave / Vivaldi):%n  1. Откроется страница расширений браузера%n  2. Включите «Режим разработчика» (Developer mode)%n  3. Нажмите «Загрузить распакованное» (Load unpacked)%n  4. Укажите папку: extension\ в директории приложения
ru.ExtStepsFF=Инструкция (Firefox):%n  1. Откроется about:debugging%n  2. Нажмите «Загрузить временное дополнение» (Load Temporary Add-on)%n  3. Укажите файл manifest.json в папке extension-firefox\ приложения%n  Примечание: расширение в Firefox загружается временно — до перезапуска браузера.
ru.ExtOpenTitle=Страницы расширений открыты
ru.ExtOpenMsg=Для каждого выбранного браузера открыта страница управления расширениями.%n%nЧтобы завершить установку:%n  1. Включите «Режим разработчика»%n  2. Нажмите «Загрузить распакованное»%n  3. Выберите папку extension\ (или extension-firefox\ для Firefox) в:%n     %1

; — English —
en.StartupEntryDesc=Launch {#AppName} on Windows startup
en.StartupEntryGroup=Autostart:
en.WebView2Status=Installing Microsoft WebView2...
en.LaunchAppDesc=Launch {#AppName}
en.ReleasesDesc=Open releases page on GitHub

en.ExtPageTitle=Browser Extension
en.ExtPageDesc=Track websites you visit
en.ExtIntroText=The FocusTrace extension sends active tab info to the desktop app (all local — nothing leaves your device).%nSelect the browsers you want to set up:
en.ExtNoBrowsers=No supported browsers were found. You can install the extension manually later from the extension\ folder inside the application directory.
en.ExtStepsChrome=Setup steps (Chrome / Edge / Opera / Yandex / Brave / Vivaldi):%n  1. The browser extensions page will open%n  2. Enable "Developer mode"%n  3. Click "Load unpacked"%n  4. Select the extension\ folder inside the app directory
en.ExtStepsFF=Setup steps (Firefox):%n  1. The about:debugging page will open%n  2. Click "Load Temporary Add-on..."%n  3. Select manifest.json inside the extension-firefox\ folder%n  Note: Firefox loads the extension temporarily — it won't survive a browser restart.
en.ExtOpenTitle=Extension pages opened
en.ExtOpenMsg=The extension management page was opened in each selected browser.%n%nTo complete the setup:%n  1. Enable "Developer mode"%n  2. Click "Load unpacked"%n  3. Select the extension\ folder (or extension-firefox\ for Firefox) inside:%n     %1

; ── Задачи при установке ─────────────────────────────────────────────────
[Tasks]
Name: desktopicon;  Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: startupentry; Description: "{cm:StartupEntryDesc}"; GroupDescription: "{cm:StartupEntryGroup}"; Flags: unchecked

; ── Файлы ─────────────────────────────────────────────────────────────────
[Files]
; Основной исполняемый файл
Source: "{#BinDir}\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion

; WebView2 bootstrapper
Source: "assets\MicrosoftEdgeWebview2Setup.exe"; DestDir: "{tmp}"; \
  Flags: deleteafterinstall; Check: not IsWebView2Installed

; Расширение для Chromium-браузеров (Chrome, Edge, Yandex, Opera, Brave, Vivaldi)
; manifest.firefox.json и скрипт сборки иконок не копируются
Source: "..\extensions\focustrace\*"; \
  DestDir: "{app}\extension"; \
  Flags: ignoreversion recursesubdirs; \
  Excludes: "manifest.firefox.json,make_icons.ps1"

; Расширение для Firefox (manifest.firefox.json → manifest.json)
Source: "..\extensions\focustrace\*"; \
  DestDir: "{app}\extension-firefox"; \
  Flags: ignoreversion recursesubdirs; \
  Excludes: "manifest.json,manifest.firefox.json,make_icons.ps1"
Source: "..\extensions\focustrace\manifest.firefox.json"; \
  DestDir: "{app}\extension-firefox"; \
  DestName: "manifest.json"; \
  Flags: ignoreversion

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
Filename: "taskkill.exe"; \
  Parameters: "/f /im {#AppExeName}"; \
  Flags: runhidden skipifdoesntexist; \
  RunOnceId: "TerminateApp"

; ── Удаление оставшихся файлов ────────────────────────────────────────────
[UninstallDelete]
Type: filesandordirs; Name: "{app}"

; ── Код ──────────────────────────────────────────────────────────────────
[Code]

// ---------------------------------------------------------------------------
// Глобальные переменные страницы расширения браузера
// ---------------------------------------------------------------------------
var
  ExtPage:         TWizardPage;
  ExtCheckList:    TNewCheckListBox;
  ExtIntroLbl:     TNewStaticText;
  ExtHintLbl:      TNewStaticText;
  // Параллельные массивы для найденных браузеров (макс. 10)
  GBrowserExe:     array[0..9] of String;  // путь к исполняемому файлу
  GBrowserUrl:     array[0..9] of String;  // URL страницы расширений
  GBrowserIsFF:    array[0..9] of Boolean; // true → Firefox (другая папка)
  GBrowserCount:   Integer;

// ---------------------------------------------------------------------------
// Проверка наличия Microsoft WebView2 (Evergreen)
// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
// Вспомогательная функция: возвращает первый существующий файл из списка
// ---------------------------------------------------------------------------
function FindFirst(const A, B, C, D: String): String;
begin
  if (A <> '') and FileExists(A) then Result := A
  else if (B <> '') and FileExists(B) then Result := B
  else if (C <> '') and FileExists(C) then Result := C
  else if (D <> '') and FileExists(D) then Result := D
  else Result := '';
end;

// ---------------------------------------------------------------------------
// Определение путей к исполняемым файлам браузеров
// ---------------------------------------------------------------------------
function EdgeExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{pf}\Microsoft\Edge\Application\msedge.exe'),
    ExpandConstant('{pf32}\Microsoft\Edge\Application\msedge.exe'),
    '', '');
end;

function ChromeExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{localappdata}\Google\Chrome\Application\chrome.exe'),
    ExpandConstant('{pf}\Google\Chrome\Application\chrome.exe'),
    ExpandConstant('{pf32}\Google\Chrome\Application\chrome.exe'),
    '');
end;

function YandexExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{localappdata}\Yandex\YandexBrowser\Application\browser.exe'),
    ExpandConstant('{pf}\Yandex\YandexBrowser\Application\browser.exe'),
    '', '');
end;

function OperaExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{localappdata}\Programs\Opera\launcher.exe'),
    ExpandConstant('{pf}\Opera\opera.exe'),
    ExpandConstant('{pf32}\Opera\opera.exe'),
    '');
end;

function OperaGXExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{localappdata}\Programs\Opera GX\launcher.exe'),
    '', '', '');
end;

function FirefoxExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{pf}\Mozilla Firefox\firefox.exe'),
    ExpandConstant('{pf32}\Mozilla Firefox\firefox.exe'),
    ExpandConstant('{localappdata}\Mozilla Firefox\firefox.exe'),
    '');
end;

function BraveExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{localappdata}\BraveSoftware\Brave-Browser\Application\brave.exe'),
    '', '', '');
end;

function VivaldiExe: String;
begin
  Result := FindFirst(
    ExpandConstant('{localappdata}\Vivaldi\Application\vivaldi.exe'),
    '', '', '');
end;

// ---------------------------------------------------------------------------
// Добавляет браузер в список, если он установлен
// ---------------------------------------------------------------------------
procedure TryAddBrowser(const Name, ExePath, ExtUrl: String; IsFF: Boolean);
begin
  if ExePath = '' then Exit;
  if GBrowserCount > 9 then Exit;
  ExtCheckList.AddCheckBox(Name, '', 0, True, True, False, False, nil);
  GBrowserExe[GBrowserCount]  := ExePath;
  GBrowserUrl[GBrowserCount]  := ExtUrl;
  GBrowserIsFF[GBrowserCount] := IsFF;
  GBrowserCount := GBrowserCount + 1;
end;

// ---------------------------------------------------------------------------
// Открывает страницу расширений выбранного браузера
// ---------------------------------------------------------------------------
procedure OpenExtPage(const ExePath, Url: String; IsFF: Boolean);
var
  RC: Integer;
  ExeName, Params: String;
begin
  ExeName := LowerCase(ExtractFileName(ExePath));
  if IsFF then
    // Firefox принимает URL как прямой аргумент
    Params := Url
  else if ExeName = 'launcher.exe' then
    // Opera launcher: прокидываем URL напрямую
    Params := Url
  else
    // Chrome / Edge / Yandex / Brave / Vivaldi: флаг --new-window
    Params := '--new-window ' + Url;
  Exec(ExePath, Params, '', SW_SHOWNORMAL, ewNoWait, RC);
end;

// ---------------------------------------------------------------------------
// Создание кастомной страницы «Расширение для браузера»
// ---------------------------------------------------------------------------
procedure CreateExtPage;
var
  Edge, Chrome, Yandex, Opera, OperaGX, Firefox, Brave, Vivaldi: String;
  HasAny: Boolean;
begin
  GBrowserCount := 0;

  ExtPage := CreateCustomPage(
    wpSelectTasks,
    ExpandConstant('{cm:ExtPageTitle}'),
    ExpandConstant('{cm:ExtPageDesc}'));

  // ── Вводный текст ────────────────────────────────────────────────────────
  ExtIntroLbl := TNewStaticText.Create(ExtPage);
  ExtIntroLbl.Parent   := ExtPage.Surface;
  ExtIntroLbl.Left     := 0;
  ExtIntroLbl.Top      := 0;
  ExtIntroLbl.Width    := ExtPage.Surface.Width;
  ExtIntroLbl.AutoSize := False;
  ExtIntroLbl.WordWrap := True;
  ExtIntroLbl.Height   := 52;
  ExtIntroLbl.Caption  := ExpandConstant('{cm:ExtIntroText}');

  // ── Список браузеров ─────────────────────────────────────────────────────
  ExtCheckList := TNewCheckListBox.Create(ExtPage);
  ExtCheckList.Parent  := ExtPage.Surface;
  ExtCheckList.Left    := 0;
  ExtCheckList.Top     := ExtIntroLbl.Top + ExtIntroLbl.Height + 6;
  ExtCheckList.Width   := ExtPage.Surface.Width;
  ExtCheckList.Height  := 152;
  ExtCheckList.BorderStyle := bsSingle;

  // Определяем и добавляем установленные браузеры
  Edge     := EdgeExe;
  Chrome   := ChromeExe;
  Yandex   := YandexExe;
  Opera    := OperaExe;
  OperaGX  := OperaGXExe;
  Firefox  := FirefoxExe;
  Brave    := BraveExe;
  Vivaldi  := VivaldiExe;

  TryAddBrowser('Microsoft Edge',    Edge,    'edge://extensions/',                      False);
  TryAddBrowser('Google Chrome',     Chrome,  'chrome://extensions/',                    False);
  TryAddBrowser('Яндекс Браузер',    Yandex,  'browser://extensions/',                   False);
  TryAddBrowser('Opera',             Opera,   'opera://extensions/',                     False);
  TryAddBrowser('Opera GX',          OperaGX, 'opera://extensions/',                     False);
  TryAddBrowser('Mozilla Firefox',   Firefox, 'about:debugging#/runtime/this-firefox',   True);
  TryAddBrowser('Brave',             Brave,   'brave://extensions/',                     False);
  TryAddBrowser('Vivaldi',           Vivaldi, 'vivaldi://extensions/',                   False);

  HasAny := GBrowserCount > 0;

  // Если ни один браузер не найден — скрываем список, показываем заглушку
  if not HasAny then
  begin
    ExtCheckList.Visible := False;
    ExtIntroLbl.Caption  := ExpandConstant('{cm:ExtNoBrowsers}');
  end;

  // ── Подсказка: шаги установки ────────────────────────────────────────────
  ExtHintLbl := TNewStaticText.Create(ExtPage);
  ExtHintLbl.Parent   := ExtPage.Surface;
  ExtHintLbl.Left     := 0;
  ExtHintLbl.Top      := ExtCheckList.Top + ExtCheckList.Height + 8;
  ExtHintLbl.Width    := ExtPage.Surface.Width;
  ExtHintLbl.AutoSize := False;
  ExtHintLbl.WordWrap := True;
  ExtHintLbl.Height   := ExtPage.Surface.Height - ExtHintLbl.Top;
  ExtHintLbl.Caption  := ExpandConstant('{cm:ExtStepsChrome}');
  ExtHintLbl.Visible  := HasAny;
end;

// ---------------------------------------------------------------------------
// InitializeWizard — создаём кастомные страницы
// ---------------------------------------------------------------------------
procedure InitializeWizard;
begin
  CreateExtPage;
end;

// ---------------------------------------------------------------------------
// CurPageChanged — обновляем подсказку, когда пользователь переходит на
// страницу расширений (путь к {app} уже может быть изменён пользователем)
// ---------------------------------------------------------------------------
procedure CurPageChanged(CurPageID: Integer);
begin
  // Страница расширений: обновляем подсказку в зависимости от браузеров
  if CurPageID = ExtPage.ID then
  begin
    if GBrowserCount > 0 then
    begin
      // Проверяем, есть ли Firefox среди выбранных браузеров
      // Показываем соответствующую подсказку (Chrome-шаги покрывают большинство)
      ExtHintLbl.Caption := ExpandConstant('{cm:ExtStepsChrome}');
    end;
  end;

  // Страница «Установка завершена»
  if CurPageID = wpFinished then
    WizardForm.FinishedLabel.Caption :=
      WizardForm.FinishedLabel.Caption + #13#10#13#10 +
      'GitHub: {#AppURL}';
end;

// ---------------------------------------------------------------------------
// CurStepChanged — после установки открываем страницы расширений
// в выбранных браузерах и показываем инструкцию
// ---------------------------------------------------------------------------
procedure CurStepChanged(CurStep: TSetupStep);
var
  I:           Integer;
  AppDir:      String;
  HasFirefox:  Boolean;
  OpenedCount: Integer;
  Msg:         String;
begin
  if CurStep <> ssPostInstall then Exit;
  if GBrowserCount = 0 then Exit;

  AppDir      := ExpandConstant('{app}');
  HasFirefox  := False;
  OpenedCount := 0;

  for I := 0 to GBrowserCount - 1 do
  begin
    if not ExtCheckList.Checked[I] then Continue;
    OpenExtPage(GBrowserExe[I], GBrowserUrl[I], GBrowserIsFF[I]);
    OpenedCount := OpenedCount + 1;
    if GBrowserIsFF[I] then HasFirefox := True;
  end;

  if OpenedCount = 0 then Exit;

  // Небольшая пауза, чтобы браузеры успели запуститься
  Sleep(800);

  // Общая инструкция со ссылкой на папку расширения
  Msg := FmtMessage(ExpandConstant('{cm:ExtOpenMsg}'), [AppDir]);
  MsgBox(Msg, mbInformation, MB_OK);

  // Firefox загружает расширения временно — показываем отдельную заметку
  if HasFirefox then
    MsgBox(ExpandConstant('{cm:ExtStepsFF}'), mbInformation, MB_OK);
end;

