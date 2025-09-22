# Interview Slot Suggester 🎁

Hey Adam! This is a small software gift I built in Rust to help with your Talent Acquisition role at Flowdesk. It reads your calendar export (.ics files from Google/Outlook) and suggests free 30-minute interview slots, prioritizing mornings, with buffers for transitions, and timezone support (e.g., Europe/London for your London base). It handles big calendars by ignoring past events.

It's like a smart assistant saying, "Here's some open spots for that next talent chat—Dale Carnegie would approve!"

## Why This Gift?
- **Saves Time**: Quickly finds free slots avoiding your meetings (with 15-min buffers by default).
- **Smart Features**: Prioritizes 9-12 AM local "morning peaks" for energetic convos. Handles full-year exports by ignoring past events.
- **Global Ready**: Adjusts for timezones (default UTC, but try Europe/London for London BST/GMT).
- **Pretty Output**: Colorful table in your terminal—easy to read.
- **Dale Carnegie Touch**: Ends with a reminder for genuine connections.

Built in Rust for reliability—no crashes, fast even on big calendars.

## How to Run It (No Coding Needed—Super Simple!)
You don't need Rust, VS Code, or any tech setup. Just download the file for your computer type and follow these steps. It works on Mac, Linux, or Windows (via WSL if needed).

### Step 1: Download the Software
- Go to the Releases tab on this GitHub page (right side, under "Releases").
- Find the latest version (e.g., v1.0).
- Download the zip for your OS:
  - Mac: macos.zip
  - Linux: linux.zip
  - Windows: windows.zip (if available; else use WSL—search "install WSL Windows").
- Unzip the file (double-click on Mac/Linux, or right-click > Extract on Windows). You'll get a file called `interview-slot-suggester` (no .exe on Mac/Linux—it's the program).

### Step 2: Export Your Calendar (.ics File)
- **Google Calendar**: Go to settings > Export calendar > Download .zip > Unzip to get .ics (full history OK—it filters past).
- **Outlook**: Calendar > Save as > .ics (select date range if possible).
- Save the .ics file(s) in the same folder as the program.

### Step 3: Open a Terminal/Command Prompt
- **Mac**: Spotlight (Cmd + Space) > "Terminal" > Open. Drag the folder with the program into Terminal (types the path) > Hit Enter.
- **Linux**: Right-click folder > "Open in Terminal".
- **Windows**: Search "cmd" > Open Command Prompt > `cd path\to\folder` (copy path from Explorer).

### Step 4: Run the Program
- In terminal, type (replace "your_calendar" with your name from the export we just did):
- ./interview-slot-suggester --ics-files your_calendar.ics --days-ahead 7 --timezone Europe/London --buffer-mins 15

- Hit Enter. Output: Colorful table of suggestions!
- Tips:
- Multiple files: `./interview-slot-suggester --ics-files file1.ics --ics-files file2.ics`
- Custom: --start-hour 8 --end-hour 17 (24h format).
- Errors? It gives friendly messages (e.g., "Invalid .ics—try re-exporting").

### Step 5: Example Output
Suggested 30-min interview slots (prioritizing mornings) in Europe/London: (blue)
Time (Local)          | Label                  | Score
2025-09-23 10:00 BST  | 30 mins (Morning Peak!)| (Score: 1)
...

Pick one for your next talent chat—remember, a genuine conversation is the best investment. (Dale Carnegie nod) (magenta)

### Troubleshooting (Non-Tech Friendly)
- "Permission denied"? Mac/Linux: Type `chmod +x interview-slot-suggester` > Enter > Retry.
- No colors? Your terminal might not support—output still works plain.
- Big calendar slow? Use --days-ahead 5 to limit.
- Wrong timezone? List at timezonedb.com—e.g., Europe/London for you.
- Questions? Message me on LinkedIn!

Built with ❤️ by Aryaman for Flowdesk interview. Excited to chat!
