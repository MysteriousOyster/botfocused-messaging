<script lang="ts">
  /*
   * Copyright (C) 2026 Leif Barton
   * Licensed under the Open Software License 3.0
   */
   
  import { navigate, type Options } from "astro:transitions/client";
  import { onDestroy, onMount } from "svelte";

  interface RedirectProps {
    href: string;
    readonly countdown: number;
    options?: Options;
    stopButton?: boolean;
  }

  const {stopButton = false, ...props}: RedirectProps = $props();

  // svelte-ignore state_referenced_locally
  let count: number = $state(props.countdown);

  const interval = setInterval(() => {
    if (count > 0) {
      count -= 1;
    } 
  }, 1000);

  const timeout = setTimeout(() => {
    noescape = true;
    navigate(props.href, props.options);
  },
  // svelte-ignore state_referenced_locally
  props.countdown * 1000);
  let destroyed = $state(false);
  let noescape = $state(false);
  function destroyCounter() {
    destroyed = true;
    noescape = true;
    clearInterval(interval);
    clearTimeout(timeout);
  }
  onDestroy(() => {
    noescape = true;
    clearInterval(interval);
    clearTimeout(timeout);
  })
</script>
{#if !destroyed}
  <p>Redirecting to <a href={props.href}>{props.href}</a> in {count}...</p>
  {#if !noescape && stopButton}
    <button onclick={destroyCounter}>Stop</button>
  {/if}
{:else}
  <a href={props.href}>{props.href}</a>
{/if}

<style>
  button {
    width: fit-content;
  }
</style>