<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	let { children, ...restProps }: HTMLAttributes<HTMLElement> = $props()
</script>

<nav {...restProps}>
	{@render children?.()}
</nav>
