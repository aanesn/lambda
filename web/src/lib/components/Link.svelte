<script lang="ts">
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import { cva, cx } from "$lib/utils"

	const linkVariants = cva({
		base: "transition-colors outline-none text-sm",
		variants: {
			intent: {
				primary: "text-neutral-500 hover:text-white"
			}
		}
	})
	type LinkProps = HTMLAnchorAttributes & {
		intent?: VariantProps<typeof linkVariants>["intent"]
		href: String
	}
	let { class: className, intent = "primary", href, children, ...restProps }: LinkProps = $props()
</script>

<a class={cx(linkVariants({ intent }), className)} {href} {...restProps}>
	{@render children?.()}
</a>
