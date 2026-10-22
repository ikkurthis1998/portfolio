// Resume PDF export. Renders the same document as the page, as the Light-only A4 layout
// (print has no dark mode), and drives the four PDF states: preparing, downloaded,
// failed and unavailable.
document.addEventListener('click', async function (event) {
    const button = event.target.closest?.('#download-resume-btn');
    if (!button || button.dataset.loading === 'true') return;
    const source = document.getElementById('resume-content');
    const status = document.getElementById('pdf-status');
    const label = button.querySelector('.btn-label');
    const setStatus = (text, state) => {
        if (!status) return;
        status.textContent = text;
        status.dataset.state = state;
    };
    if (!source || typeof html2pdf === 'undefined') {
        setStatus('PDF export is unavailable. Use your browser’s Print menu to save a PDF.', 'info');
        return;
    }
    button.dataset.loading = 'true';
    button.setAttribute('aria-busy', 'true');
    if (label) label.textContent = 'Preparing PDF…';
    setStatus('Preparing PDF…', 'info');
    const container = document.createElement('div');
    container.className = 'theme-light resume-pdf';
    container.style.cssText = 'width:794px;background:#ffffff;';
    const documentCopy = source.cloneNode(true);
    documentCopy.removeAttribute('id');
    documentCopy.style.cssText = 'padding-top:0;padding-bottom:0;';
    container.appendChild(documentCopy);
    // Keep each section label on the same page as its first entry.
    documentCopy.querySelectorAll('.resume-list > .eyebrow').forEach(label => {
        const keep = document.createElement('div');
        keep.className = 'resume-keep';
        label.before(keep);
        keep.append(label);
        if (keep.nextElementSibling) keep.append(keep.nextElementSibling);
    });
    try {
        await document.fonts.ready;
        // html2canvas does not resolve currentColor in inline SVG; bake the colour in.
        documentCopy.querySelectorAll('.resume-contact svg').forEach(svg => {
            const color = '#5c5c5c'; // text/secondary (Light): the PDF is Light only
            svg.setAttribute('xmlns', 'http://www.w3.org/2000/svg');
            svg.querySelectorAll('path').forEach(path => {
                path.setAttribute('stroke', color);
                path.setAttribute('fill', 'none');
                path.setAttribute('stroke-width', '2.625');
                path.setAttribute('stroke-linecap', 'round');
                path.setAttribute('stroke-linejoin', 'round');
                path.removeAttribute('vector-effect');
            });
            const markup = new XMLSerializer().serializeToString(svg);
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
            margin: [12.7, 0, 12.7, 0],
            image: { type: 'jpeg', quality: 0.98 },
            html2canvas: { scale: 2, backgroundColor: '#ffffff' },
            jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' },
            pagebreak: { mode: ['css', 'legacy'], avoid: ['.resume-identity', '.resume-keep', '.resume-entry', '.resume-project', '.resume-section'] },
            enableLinks: true
        }).from(container).save();
        setStatus('PDF downloaded.', 'done');
    } catch (error) {
        console.error('PDF export failed', error);
        setStatus('Could not create the PDF. Use your browser’s Print menu to save a PDF.', 'error');
    } finally {
        container.remove();
        delete button.dataset.loading;
        button.removeAttribute('aria-busy');
        if (label) label.textContent = 'Download PDF';
    }
});
