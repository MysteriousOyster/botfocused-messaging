<script lang="ts">
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */

  import { amISignedIn, createUser } from "../../api.svelte";
  import { navigate } from "astro:transitions/client";
  import Redirect from "../bits/Redirect.svelte";

  let username = $state("");
  let password = $state("");
  let disabled = $state(false);
  let error: undefined | string = $state(undefined);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    disabled = true;
    try {
      const res = await createUser(username, password);
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

<h1>Create a New Account</h1>
{#await amISignedIn()}
  <p>Loading form...</p>
{:then signedIn}
  {#if !signedIn}
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
      <button type="submit" {disabled}>Do it</button>
    </form>
    <p>
      By creating an account you agree to having a cookie stored on your device
      so we know you&rsquo;re logged in.
    </p>
    {#if error}
      <p style:color="var(--error)">{error}</p>
    {/if}
  {:else}
    <p>Looks like you&rsquo;re already signed in.</p>
    <Redirect href="/account/me" countdown={5}/>
  {/if}
{:catch}
  <p>
    Uh oh! We can&rsquo;t connect right now. Check your internet connection and
    try again in a few minutes.
  </p>
{/await}
