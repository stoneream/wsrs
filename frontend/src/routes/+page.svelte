<script lang="ts">
  import init, { Client } from 'client';
  import { onMount } from 'svelte';

  let client = null as Client | null;
  let roomId = "";
  let message = "";
  
  onMount(async () => {
    await init();
    client = new Client("ws://localhost:9001");
  });

  function createRoom() {
    client?.create_room();
  }

  function joinRoom() {
    client?.join_room(roomId);
  }

  function leaveRoom() {
    client?.leave_room();
  }

  function sendMessage() {
    client?.send_message(message);
  }
</script>

<h1>wsrs</h1>

<button on:click={createRoom}>Create Room</button>

<div>
  <label for="room-id">Room ID</label>
  <input type="text" bind:value={roomId} />
</div>

<div>
  <button on:click={joinRoom}>Join Room</button>
  <button on:click={leaveRoom}>Leave Room</button>
</div>

<div>
  <label for="message">Message</label>
  <input type="text" bind:value={message} />
  <button on:click={sendMessage}>Send Message</button>
</div>

