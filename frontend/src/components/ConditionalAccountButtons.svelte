<script lang="ts">
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */
   
  import { amISignedIn } from "../api.svelte";
  import LinkButton from "./LinkButton.svelte";
</script>
{#await amISignedIn()}
	<p>Loading account status...</p>
{:then yesno}
{#if yesno}
  <LinkButton href="/account/me">My Account</LinkButton>
{:else}
  <LinkButton href="/account/login">Log In</LinkButton>
	<LinkButton href="/account/new">Create Account</LinkButton>
{/if}
{:catch e}
	<p style:color="var(--error)">Error loading account status: {e?.message}</p>
{/await}