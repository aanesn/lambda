<script lang="ts">
	import Header from "$lib/components/Header.svelte"
	let { children } = $props()
</script>

<div class="container mx-auto scroll-smooth px-6 lg:max-w-5xl">
	<Header />
	{@render children?.()}
</div>
