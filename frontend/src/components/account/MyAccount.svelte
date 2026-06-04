<script lang="ts">
    import { navigate } from "astro:transitions/client";
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */

  import { myAccount, PermissionLevelLabel, logOut } from "../../api.svelte";
  import Redirect from "../bits/Redirect.svelte";

  let logOutPromise: null | Promise<Response> = $state(null);

  function goToHome() {
    navigate("/");
  }
</script>

{#await myAccount()}
  <p>Loading acccount details...</p>
{:then account}
  <h1>Hi, {account.username}!</h1>
  <p>
    Your permission is {PermissionLevelLabel[account.permission]} and id is {account.id}.
  </p>
  <button
    onclick={() => (logOutPromise = logOut())}
    disabled={logOutPromise !== null}>Log Out</button
  >
  {#if logOutPromise}
    {#await logOutPromise}
      <p>Logging out...</p>
    {:then response}
      {#if response.ok}
        {goToHome()}
      {:else}
        {() => logOutPromise = null}
        <p style:color="var(--error)">
          Failed to log out {#await response.text() then text}
            {text}
          {/await}
        </p>
      {/if}
    {:catch e}
      <p style:color="var(--error)">Failed to log out: {e?.message}</p>
    {/await}
  {/if}
{:catch e}
  <p style:color="var(--error)">Failed to fetch account. {e?.message || e}</p>
  <Redirect href="/account/login" countdown={5} stopButton/>
{/await}
