<script lang="ts">
	import Header from "$lib/components/ui/Header.svelte"
	import "../app.css"

	let { children } = $props()
</script>

<div class="container mx-auto scroll-smooth px-6 lg:px-16 xl:max-w-7xl">
	<Header />
	{@render children?.()}
</div>
