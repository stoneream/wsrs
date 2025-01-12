<script>
  import init, { Client } from 'client';
  import { onMount } from 'svelte';
  
  onMount(async () => {
  
    await init();
  
    const client = new Client("ws://localhost:9001");


  });
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
