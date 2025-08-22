<script lang="ts">
	import { cva, cx } from "$lib/utils/cva.config"
	import type { VariantProps } from "cva"
	import type { HTMLButtonAttributes } from "svelte/elements"

	const button = cva({
		base: "inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-medium outline-none transition-all",
		variants: {
			intent: {
				primary: "shadow-xs bg-white text-black hover:bg-white/85"
			},
			size: {
				md: "h-8 px-4 py-2"
			}
		}
	})

	type Variants = VariantProps<typeof button>
	type Intent = Variants["intent"]
	type Size = Variants["size"]

	type Props = HTMLButtonAttributes & {
		intent?: Intent
		size?: Size
	}

	let {
		class: className,
		children,
		intent = "primary",
		size = "md",
		...restProps
	}: Props = $props()
</script>

<button class={cx(button({ intent, size }), className)} {...restProps}>
	{@render children?.()}
</button>
