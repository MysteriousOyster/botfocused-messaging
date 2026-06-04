<script lang="ts">
  /*
  * Copyright (C) 2026 Leif Barton
  * Licensed under the Open Software License 3.0
  */

  let {children, ...props} = $props();
</script>

<a {...props}>{@render children()}</a>
<style>
  a,
  a:link,
  a:visited,
  a:hover,
  a:active,
  a:focus {
    color: var(--txt-main);
    text-decoration: none;
  }
  a {
    border: 2px var(--border-main) solid;
    width: fit-content;
    padding: 2px 4px;
    border-radius: 7px;
    background-color: var(--bg-dark);
  }
</style>
