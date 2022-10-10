import { useState, useEffect, useCallback } from 'react';

export function useCoinInputDisplayValue(
  initialValue: string,
  onChange: (val: string) => void
): [string, (e: React.ChangeEvent<HTMLInputElement>) => void] {
  const [value, setValue] = useState(initialValue);

  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  useEffect(() => {
    if (value !== initialValue) onChange?.(value);
  }, [value]);

  const valueSetter = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const valueWithoutLeadingZeros = e.target.value.replace(/^0+\d/, (substring) =>
        substring.replace(/^0+(?=[\d])/, '')
      );

      setValue(
        valueWithoutLeadingZeros.startsWith('.')
          ? `0${valueWithoutLeadingZeros}`
          : valueWithoutLeadingZeros
      );
    },
    [setValue]
  );

  return [value, valueSetter];
}
