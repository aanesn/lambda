<script lang="ts" module>
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import { cva, cx } from "$lib/utils"

	export const linkVariants = cva({
		base: "inline-flex shrink-0 items-center justify-center rounded-full text-sm whitespace-nowrap transition-all outline-none",
		variants: {
			intent: {
				primary: "text-white hover:text-white/80",
				underline: "text-white underline hover:text-white/80"
			}
		}
	})
	export type LinkIntent = VariantProps<typeof linkVariants>["intent"]
	export type LinkProps = HTMLAnchorAttributes & {
		intent?: LinkIntent
		href: String
	}
</script>

<script lang="ts">
	let { class: className, intent = "primary", href, children, ...restProps }: LinkProps = $props()
</script>

<a class={cx(linkVariants({ intent }), className)} {href} {...restProps}>
	{@render children?.()}
</a>
