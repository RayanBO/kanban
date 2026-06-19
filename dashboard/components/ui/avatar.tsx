"use client"

import { Avatar as AvatarPrimitive } from "radix-ui"
import { cn } from "@/lib/utils"

const Avatar = ({
  className,
  ...props
}: React.ComponentProps<typeof AvatarPrimitive.Root>) => (
  <AvatarPrimitive.Root
    className={cn("relative flex h-6 w-6 shrink-0 overflow-hidden rounded-full", className)}
    {...props}
  />
)

const AvatarImage = ({
  className,
  ...props
}: React.ComponentProps<typeof AvatarPrimitive.Image>) => (
  <AvatarPrimitive.Image
    className={cn("aspect-square h-full w-full", className)}
    {...props}
  />
)

const AvatarFallback = ({
  className,
  ...props
}: React.ComponentProps<typeof AvatarPrimitive.Fallback>) => (
  <AvatarPrimitive.Fallback
    className={cn(
      "flex h-full w-full items-center justify-center rounded-full bg-muted text-xs font-medium text-muted-foreground",
      className
    )}
    {...props}
  />
)

export { Avatar, AvatarImage, AvatarFallback }
