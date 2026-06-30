export function HeaderTable({ rows }: { rows: [string, string][] }) {
  return (
    <table className="w-full text-xs">
      <tbody>
        {rows.map(([key, value], i) => (
          <tr
            className="align-top"
            key={`${key}-${
              // biome-ignore lint/suspicious/noArrayIndexKey: stable
              i
            }`}
          >
            <td className="whitespace-nowrap py-1 pr-3 font-medium text-muted-foreground">
              {key}
            </td>
            <td className="break-all py-1 font-mono">{value}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
