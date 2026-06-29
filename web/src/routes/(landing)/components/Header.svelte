<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
</script>

<header class="flex items-center justify-between h-16">
	<a href="/">
		{@html Logomark}
	</a>
	<Button class="duration-300">start now</Button>
</header>
