<script lang="ts">
  import { isApiHealthy } from "../api";
</script>

{#await isApiHealthy()}
  <p>API status: loading...</p>
{:then health}
  <p>API status: <span data-health={health}>{health ? "healthy" : "diseased"}</span></p>
{/await}

<style>
  span[data-health] {
    font-weight: bold;
  }
  span[data-health = false] {
    color: #e55;
  }
  span[data-health = true] {
    color: #595;
  }
</style>