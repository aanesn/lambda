<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLElement> = $props()
</script>

<main class={cx("bg-background relative flex w-full flex-1 flex-col", className)} {...restProps}>
	{@render children?.()}
</main>
