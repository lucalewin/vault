export const copyToClipboard = (text: string) => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text).then(() => {
      // alert('Copied to clipboard!');
    }).catch(err => {
      console.error('Failed to copy:', err);
    });
  } else {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.select();
    try {
      document.execCommand('copy');
      // alert('Copied to clipboard!');
    } catch (err) {
      console.error('Failed to copy:', err);
    }
    document.body.removeChild(textArea);
  }
};

export function download(filename: string, text: string) {
  var pom = document.createElement('a');
  pom.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
  pom.setAttribute('download', filename);

  if (document.createEvent) {
      var event = document.createEvent('MouseEvents');
      event.initEvent('click', true, true);
      pom.dispatchEvent(event);
  }
  else {
      pom.click();
  }
}

export const generatePassword = () => {
  const upper = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
  const lower = 'abcdefghijklmnopqrstuvwxyz';
  const numbers = '0123456789';
  const symbols = '!@#$%^&*()_+[]{}|;:,.<>?';
  const allChars = upper + lower + numbers + symbols;

  let generatedPassword = '';
  generatedPassword += upper.charAt(Math.floor(Math.random() * upper.length));
  generatedPassword += lower.charAt(Math.floor(Math.random() * lower.length));
  generatedPassword += numbers.charAt(Math.floor(Math.random() * numbers.length));
  generatedPassword += symbols.charAt(Math.floor(Math.random() * symbols.length));

  for (let i = 4; i < 25; i++) {
    generatedPassword += allChars.charAt(Math.floor(Math.random() * allChars.length));
  }

  return generatedPassword.split('').sort(() => 0.5 - Math.random()).join('');
};
