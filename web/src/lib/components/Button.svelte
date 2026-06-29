<script lang="ts">
	import { cva, cx } from "$lib/utils"
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements"

	const button = cva({
		base: "inline-flex shrink-0 cursor-pointer items-center justify-center rounded-xl text-sm whitespace-nowrap transition-all select-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			intent: {
				primary: "bg-white text-black hover:bg-white/80"
			},
			size: {
				md: "h-8 px-3"
			}
		}
	})

	let {
		class: className,
		children,
		intent = "primary",
		size = "md",
		href = undefined,
		type = "button",
		...others
	}: HTMLButtonAttributes &
		HTMLAnchorAttributes & {
			intent?: VariantProps<typeof button>["intent"]
			size?: VariantProps<typeof button>["size"]
		} = $props()
</script>

{#if href}
	<a {href} class={cx(button({ intent, size }), className)} {...others}>
		{@render children?.()}
	</a>
{:else}
	<button class={cx(button({ intent, size }), className)} {type} {...others}>
		{@render children?.()}
	</button>
{/if}
