<script lang="ts">
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */
   
  import { isApiHealthy } from "../api";
</script>

{#await isApiHealthy()}
  <p>API status: loading... <noscript style:color="var(--error)">Enable Javascript to check API health</noscript></p>
{:then health}
  <p>API status: <span data-health={health}>{health ? "healthy" : "diseased"}</span></p>
{/await}

<style>
  p {
    margin: 0;
  }
  span[data-health = false] {
    color: var(--error);
    font-weight: bold;
  }
  span[data-health = true] {
    color: inherit;
  }
</style>