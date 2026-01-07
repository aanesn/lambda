<script lang="ts">
	import Header from "./components/Header.svelte"

	let { children } = $props()
</script>

<div class="container mx-auto px-6 lg:px-20 xl:max-w-7xl">
	<Header />
	{@render children()}
</div>
