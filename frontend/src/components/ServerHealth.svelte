<script lang="ts">
  import { isApiHealthy } from "../api";
</script>

{#await isApiHealthy()}
  <p>API status: loading... <noscript style:color="var(--error)">Enable Javascript to check API health</noscript></p>
{:then health}
  <p>API status: <span data-health={health}>{health ? "healthy" : "diseased"}</span></p>
{/await}

<style>
  span[data-health] {
    font-weight: bold;
  }
  span[data-health = false] {
    color: var(--error);
  }
  span[data-health = true] {
    color: var(--success);
  }
</style>