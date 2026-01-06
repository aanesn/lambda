<script lang="ts">
	import type { HTMLLiAttributes } from "svelte/elements"

	let { ...restProps }: HTMLLiAttributes = $props()
</script>

<li {...restProps}>/</li>
