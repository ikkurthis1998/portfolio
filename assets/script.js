// Wait for DOM and handle events
document.addEventListener('DOMContentLoaded', function () {
    setupDownloadButton();
});

// Also listen for potential Leptos navigations/updates if needed, 
// but event delegation on document body is safest for dynamic content
document.addEventListener('click', function (e) {
    if (e.target && (e.target.id === 'download-resume-btn' || e.target.closest('#download-resume-btn'))) {
        downloadResume();
    }
});

function setupDownloadButton() {
    const btn = document.getElementById('download-resume-btn');
    if (btn) {
        btn.onclick = downloadResume;
    }
}

function downloadResume() {
    console.log('Download triggered via event listener');
    const element = document.getElementById('resume-content');

    // Check if element exists
    if (!element) {
        console.error('Resume content element not found!');
        return;
    }

    // Check if html2pdf is loaded
    if (typeof html2pdf === 'undefined') {
        console.error('html2pdf library not loaded!');
        return;
    }

    const opt = {
        margin: 0,
        filename: 'Sreemannarayana_Ikkurthi_Resume.pdf',
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: { scale: 2, useCORS: true },
        jsPDF: { unit: 'in', format: 'letter', orientation: 'portrait' }
    };

    html2pdf().set(opt).from(element).save().then(() => {
        console.log('PDF saved successfully');
    }).catch(err => {
        console.error('PDF save failed:', err);
    });
}
