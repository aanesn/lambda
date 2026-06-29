<script lang="ts">
	import Header from "./components/Header.svelte"

	let { children } = $props()
</script>

<div class="mx-auto container px-6 xl:max-w-7xl">
	<Header />
	{@render children()}
</div>
