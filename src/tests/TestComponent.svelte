<script lang="ts">
  let { count = $bindable(0), message = "" }: { count?: number; message?: string } = $props();

  function increment() {
    count++;
  }
</script>

<div class="test-component">
  <h1>{message}</h1>
  <p>Count: {count}</p>
  <button onclick={increment}>Increment</button>
</div>
