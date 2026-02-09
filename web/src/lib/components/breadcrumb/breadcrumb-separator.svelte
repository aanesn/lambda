<script lang="ts">
	import type { HTMLLiAttributes } from "svelte/elements"

	let { class: className, ...restProps }: HTMLLiAttributes = $props()
</script>

<li {...restProps}>/</li>
