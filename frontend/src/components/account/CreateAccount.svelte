<script lang="ts">
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */

  import { API_URL } from "../../config";
  import { isApiHealthy, createUser } from "../../api";
  let username = $state("");
  let password = $state("");

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    const res = await createUser(username, password);
    if (!res.ok) {
      console.error(res, res.text());
    } else {
      console.log(res);
    }
  }
</script>

<h1>Create a New Account</h1>
{#await isApiHealthy()}
	<p>Loading form...</p>
{:then healthy} 
{#if healthy}
  <form onsubmit={handleSubmit}>
    <input type="text" bind:value={username} placeholder="username" />
    <input type="password" bind:value={password} placeholder="password" />
    <button type="submit">Do it</button>
  </form>
{:else}
  <p>Uh oh! We can&rsquo;t connect right now. Check your internet connection and try again in a few minutes.</p>
{/if}
{/await}
