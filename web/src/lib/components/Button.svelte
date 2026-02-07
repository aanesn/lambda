<script lang="ts">
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements"
	import { cva, cx } from "$lib/utils"

	const buttonVariants = cva({
		base: "inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			intent: {
				primary: "bg-white text-black hover:bg-white/80 shadow-xs font-medium",
				secondary: "bg-neutral-900 hover:bg-neutral-800",
				ghost: "hover:bg-neutral-900",
				outline: "border shadow-xs bg-neutral-900 hover:bg-neutral-800"
			},
			size: {
				md: "h-10 px-5 py-2 rounded-full gap-2 text-sm",
				lg: "h-13 px-6 py-2 rounded-full gap-3",
				icon: "size-8 rounded-lg"
			}
		}
	})
	type ButtonProps = HTMLButtonAttributes &
		HTMLAnchorAttributes & {
			intent?: VariantProps<typeof buttonVariants>["intent"]
			size?: VariantProps<typeof buttonVariants>["size"]
		}
	let {
		class: className,
		intent = "primary",
		size = "md",
		href = undefined,
		children,
		...restProps
	}: ButtonProps = $props()
</script>

{#if href}
	<a class={cx(buttonVariants({ intent, size }), className)} {href} {...restProps}>
		{@render children?.()}
	</a>
{:else}
	<button class={cx(buttonVariants({ intent, size }), className)} {...restProps}>
		{@render children?.()}
	</button>
{/if}
