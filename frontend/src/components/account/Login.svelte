<script lang="ts">
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */

  import { isApiHealthy, logIn } from "../../api.svelte";
  import { navigate } from "astro:transitions/client";

  let username = $state("");
  let password = $state("");
  let disabled = $state(false);
  let error: undefined | string = $state(undefined);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    disabled = true;
    try {
      const res = await logIn(username, password);
      console.log(res);
      if (!res.ok) {
        console.error(res);
        error = await res.text();
        disabled = false;
      } else {
        console.log(res);
        navigate("/account/me");
      }
    } catch (e) {
      console.error(e);
      error = "Unable to contact API. Please try again later.";
      disabled = false;
    }
  }
</script>

<h1>Log In To Account</h1>
{#await isApiHealthy()}
  <p>Loading form...</p>
{:then healthy}
  {#if healthy}
    <form onsubmit={handleSubmit}>
      <input
        type="text"
        bind:value={username}
        placeholder="username"
        {disabled}
      />
      <input
        type="password"
        bind:value={password}
        placeholder="password"
        {disabled}
      />
      <button type="submit" {disabled}>Log In</button>
    </form>
    <p>Don&rsquo;t have an account? <a href="/account/new">Create one</a></p>
    <p>
      By signing in you agree to having a cookie stored on your device so we
      know you&rsquo;re logged in.
    </p>
    {#if error}
      <p style:color="var(--error)">{error}</p>
    {/if}
  {:else}
    <p>
      Uh oh! We can&rsquo;t connect right now. Check your internet connection
      and try again in a few minutes.
    </p>
  {/if}
{/await}
