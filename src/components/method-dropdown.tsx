import { cn } from "@/lib/utils";
import {
  ALL_METHODS,
  type HttpMethod,
  METHOD_BG_COLORS,
  METHOD_COLORS,
} from "@/lib/constants";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";

interface MethodDropdownProps {
  method: HttpMethod;
  onChange: (method: HttpMethod) => void;
}

export function MethodDropdown({ method, onChange }: MethodDropdownProps) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Button
            variant="outline"
            size="sm"
            className={cn(
              "min-w-[62px] gap-1 font-semibold tracking-[0.05em]",
              METHOD_COLORS[method],
              METHOD_BG_COLORS[method],
              "border-current/25",
            )}
          />
        }
      >
        {method}
        <svg width="7" height="4" viewBox="0 0 7 4" className="ml-0.5">
          <path
            d="M1 1l2.5 2L6 1"
            stroke="currentColor"
            strokeWidth="1.2"
            strokeLinecap="round"
            fill="none"
          />
        </svg>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        {ALL_METHODS.map((m) => (
          <DropdownMenuItem
            key={m}
            onClick={() => onChange(m)}
            className={cn(
              "font-semibold tracking-[0.04em]",
              METHOD_COLORS[m],
              m === method && "bg-muted",
            )}
          >
            {m}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
