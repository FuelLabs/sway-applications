import { toast } from '@fuel-ui/react';
import { isB256 } from 'fuels';

export function validateData(data: string) {
  let isError = false;

  if (data === '') {
    toast.error('Hey... like, uhm, you know the data is empty, right?', {
      duration: 10000,
    });
    isError = true;
  } else if (!isB256(data)) {
    toast.error("I don't know about that data format chief", {
      duration: 10000,
    });
    isError = true;
  }

  return { data, isError };
}
