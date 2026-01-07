<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: DropdownMenu.ItemProps = $props()
</script>

<DropdownMenu.Item
	class={cx(
		"relative flex cursor-default items-center gap-2 rounded-lg px-2 py-1.5 text-sm outline-hidden select-none data-highlighted:bg-neutral-900 [&_svg]:pointer-events-none [&_svg]:shrink-0",
		className
	)}
	{...restProps}
/>
