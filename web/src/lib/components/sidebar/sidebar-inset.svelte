<script lang="ts">
	import { cx } from "$lib/utils"
	import type { HTMLAttributes } from "svelte/elements"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLElement> = $props()
</script>

<main class={cx("relative flex w-full flex-1 flex-col bg-black", className)} {...restProps}>
	{@render children?.()}
</main>
