<script lang="ts">
    import { onMount } from 'svelte';

    let workDuration: number = 25; // Default work duration in minutes
    let breakDuration: number = 5; // Default break duration in minutes
    let remainingTime: number = 0; // Remaining time in seconds
    let timer: NodeJS.Timeout | null = null;
    let isWorkSession: boolean = true;
    let isTimerRunning: boolean = false;

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
            isWorkSession = !isWorkSession;
            alert(isWorkSession ? 'Break over! Time to work.' : 'Work session over! Time for a break.');
        }
        }, 1000);
    }

    // Cancel the timer
    function cancelTimer() {
        if (timer) {
        clearInterval(timer);
        timer = null;
        }
        isTimerRunning = false;
        remainingTime = 0;
    }

    // Extend the current session by a specified number of minutes
    function extendTimer() {
        const extensionMinutes = parseInt(prompt('Enter extension time in minutes:', '5') || '0');
        if (!isNaN(extensionMinutes) && extensionMinutes > 0) {
        remainingTime += extensionMinutes * 60;
        } else {
        alert('Invalid input. Please enter a positive number.');
        }
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
        <button on:click={startTimer} disabled={isTimerRunning}>Start</button>
        <button on:click={cancelTimer} disabled={!isTimerRunning}>Cancel</button>
        <button on:click={extendTimer} disabled={!isTimerRunning}>Extend</button>
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