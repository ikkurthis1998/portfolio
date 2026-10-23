// Export the same document and styles shown on the resume page.
document.addEventListener('click', async function (event) {
    const button = event.target.closest?.('#download-resume-btn');
    if (!button || button.disabled) return;
    const source = document.getElementById('resume-content');
    const status = document.getElementById('pdf-status');
    if (!source || typeof html2pdf === 'undefined') {
        if (status) status.textContent = 'PDF export is unavailable. Use your browser’s Print menu to save a PDF.';
        return;
    }
    button.disabled = true;
    if (status) status.textContent = 'Preparing PDF…';
    const container = document.createElement('div');
    container.className = 'classic-resume resume-pdf-document';
    container.style.cssText = 'width:794px;padding:0;background:white;';
    const documentCopy = source.cloneNode(true);
    documentCopy.style.cssText = 'width:794px;max-width:none;margin:0;padding:0 32px;background:white;box-shadow:none;';
    container.appendChild(documentCopy);
    try {
        await document.fonts.ready;
        await Promise.all(Array.from(source.querySelectorAll('img'), image => image.decode()));
        // html2canvas does not preserve object-fit or currentColor SVGs reliably.
        const portrait = source.querySelector('.resume-portrait');
        if (portrait) {
            const canvas = document.createElement('canvas');
            canvas.width = canvas.height = 384;
            const side = Math.min(portrait.naturalWidth, portrait.naturalHeight);
            canvas.getContext('2d').drawImage(portrait, (portrait.naturalWidth - side) / 2, 0, side, side, 0, 0, 384, 384);
            documentCopy.querySelector('.resume-portrait').src = canvas.toDataURL('image/png');
        }
        const originals = source.querySelectorAll('.resume-contact svg');
        documentCopy.querySelectorAll('.resume-contact svg').forEach((svg, index) => {
            const color = getComputedStyle(originals[index]).color;
            svg.setAttribute('xmlns', 'http://www.w3.org/2000/svg');
            svg.setAttribute('width', '16');
            svg.setAttribute('height', '16');
            const markup = new XMLSerializer().serializeToString(svg).replace(/currentColor/g, color);
            const icon = document.createElement('img');
            icon.src = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(markup);
            icon.width = icon.height = 16;
            icon.alt = '';
            icon.style.cssText = 'width:16px;height:16px;flex-shrink:0;';
            svg.replaceWith(icon);
        });
        await Promise.all(Array.from(documentCopy.querySelectorAll('img'), image => image.decode()));
        await html2pdf().set({
            filename: 'Sreemannarayana_Ikkurthi_Resume.pdf',
            margin: [8.47, 0, 8.47, 0],
            image: { type: 'jpeg', quality: 0.98 },
            html2canvas: { scale: 2, backgroundColor: '#ffffff' },
            jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' },
            pagebreak: { mode: ['css', 'legacy'], avoid: ['.resume-identity', '.resume-entry', '.resume-project-entry', '.resume-skills', '.resume-summary'] },
            enableLinks: true
        }).from(container).save();
        if (status) status.textContent = 'PDF downloaded.';
    } catch (error) {
        console.error('PDF export failed', error);
        if (status) status.textContent = 'Could not create the PDF. Use your browser’s Print menu to save a PDF.';
    } finally {
        container.remove();
        button.disabled = false;
    }
});
