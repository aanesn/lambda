<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLDivElement> = $props()
</script>

<div class="relative flex min-h-screen items-center justify-center overflow-hidden">
	<div class={cx("absolute top-45 w-full max-w-xl", className)} {...restProps}>
		{@render children?.()}
	</div>
</div>
