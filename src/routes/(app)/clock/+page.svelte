<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    let workDuration: number = 25; // Default work duration in minutes
    let breakDuration: number = 5; // Default break duration in minutes
    let remainingTime: number = 0; // Remaining time in seconds
    let timer: NodeJS.Timeout | null = null;
    let isWorkSession: boolean = true;
    let isTimerRunning: boolean = false;

    let isExtended: boolean = false; // Indicates if the session is currently extended
    let extensionStartTime: Date | null = null; // Records the start time of the extension
    let totalWorkDuration: number = workDuration * 60; // Tracks the total work duration

    // Format time as MM:SS
    function formatTime(seconds: number): string {
        const minutes = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
    }

    // Start the timer
    function startTimer() {
        if (isTimerRunning) return;
        isTimerRunning = true;
        remainingTime = isWorkSession ? workDuration * 60 : breakDuration * 60;
        timer = setInterval(() => {
            if (remainingTime > 0) {
                remainingTime--;
            } else {
                clearInterval(timer!);
                isTimerRunning = false;
                if (isWorkSession) {
                    // Work session ended, prompt user action
                    alert('Work session over! Choose "Extend" to continue or "Start Break" to proceed.');
                } else {
                    // Break session ended
                    saveSession();
                    alert('Break over! Time to work.');
                    isWorkSession = true; // Reset for next work session
                }
            }
        }, 1000);
    }

    // // Save session data
    // function saveSession() {
    //     const startTimeWork = new Date().toISOString(); // This should track actual session start time
    //     const stopTimeWork = new Date().toISOString(); // This should track actual session stop time
    //     invoke('save_timer_session', {
    //         workDuration: totalWorkDuration,
    //         breakDuration: breakDuration * 60,
    //         startTimeWork: startTimeWork,
    //         stopTimeWork: stopTimeWork,
    //         extended: isExtended,
    //         extendedStartTime: extensionStartTime ? extensionStartTime.toISOString() : null,
    //         extendedStopTime: new Date().toISOString(),
    //     });
    //     resetTimer();
    // }

    function saveSession() {
        const startTimeWork = new Date().toISOString(); // Track actual session start time
        const stopTimeWork = new Date(new Date().getTime() + totalWorkDuration * 1000).toISOString(); // Simulate stop time
        const startTimeBreak = isWorkSession ? null : new Date().toISOString(); // Break starts after work session
        const stopTimeBreak = isWorkSession
            ? null
            : new Date(new Date().getTime() + breakDuration * 60 * 1000).toISOString(); // Simulate break stop time

        // Wrap the session object under the `session` key
        invoke('save_timer_session_command', {
            session: {
                work_duration: workDuration, // Snake case
                break_duration: breakDuration, // Snake case
                start_time_work: startTimeWork, // Snake case
                stop_time_work: stopTimeWork, // Snake case
                start_time_break: startTimeBreak, // Snake case
                stop_time_break: stopTimeBreak, // Snake case
                extended: isExtended,
                extended_start_time: extensionStartTime ? extensionStartTime.toISOString() : null, // Snake case
                extended_stop_time: isExtended ? new Date().toISOString() : null, // Snake case
            },
        })
            .then(() => {
                console.log('Session saved successfully.');
            })
            .catch((err) => {
                console.error('Failed to save session:', err);
            });
        
        console.log({
            session: {
                work_duration: workDuration, // Snake case
                break_duration: breakDuration, // Snake case
                start_time_work: startTimeWork, // Snake case
                stop_time_work: stopTimeWork, // Snake case
                start_time_break: startTimeBreak, // Snake case
                stop_time_break: stopTimeBreak, // Snake case
                extended: isExtended,
                extended_start_time: extensionStartTime ? extensionStartTime.toISOString() : null, // Snake case
                extended_stop_time: isExtended ? new Date().toISOString() : null, // Snake case
            },
        })
        resetTimer();
    }

    // Reset timer for the next session
    function resetTimer() {
        remainingTime = 0;
        isTimerRunning = false;
        isExtended = false;
        extensionStartTime = null;
        totalWorkDuration = workDuration * 60;
    }

    // Start the extension
    function extendTimer() {
        if (isTimerRunning || isExtended) return;
        isExtended = true;
        extensionStartTime = new Date();
        alert('Extension started. Press "End Extension" to finalize.');
    }

    // End the extension
    function endExtension() {
        if (!isExtended || !extensionStartTime) return;
        const extensionEndTime = new Date();
        const extensionDuration = Math.floor((extensionEndTime.getTime() - extensionStartTime.getTime()) / 1000); // in seconds
        totalWorkDuration += extensionDuration;
        isExtended = false;
        extensionStartTime = null;
        isWorkSession = false;
        alert(`Extension ended. You extended your session by ${formatTime(extensionDuration)}.`);
    }

    // Start the break
    function startBreak() {
        if (isTimerRunning) return;
        isWorkSession = false;
        startTimer();
    }
</script>

<main class="container">
    <h1>Pomodoro Timer</h1>
    <div class="input-group">
        <label for="workDuration">Work Duration (minutes):</label>
        <input type="number" id="workDuration" bind:value={workDuration} min="1" />
    </div>
    <div class="input-group">
        <label for="breakDuration">Break Duration (minutes):</label>
        <input type="number" id="breakDuration" bind:value={breakDuration} min="1" />
    </div>
    <div class="timer-display">
        {formatTime(remainingTime)}
    </div>
    <div class="button-group">
        <button on:click={startTimer} disabled={isTimerRunning || isExtended}>Start</button>
        <button on:click={extendTimer} disabled={isTimerRunning || !isWorkSession}>Extend</button>
        <button on:click={endExtension} disabled={!isExtended}>End Extension</button>
        <button on:click={startBreak} disabled={isTimerRunning || !isWorkSession}>Start Break</button>
    </div>
</main>

<style>
    .container {
        text-align: center;
        margin-top: 50px;
    }
    .input-group {
        margin: 10px 0;
    }
    label {
        margin-right: 10px;
    }
    input {
        width: 50px;
        text-align: center;
        color: black;
    }
    .timer-display {
        font-size: 48px;
        margin: 20px 0;
    }
    .button-group button {
        margin: 5px;
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
    }
    button:disabled {
        background-color: #ccc;
        color: black;
        cursor: not-allowed;
    }
</style>