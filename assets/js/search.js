document.addEventListener('DOMContentLoaded', function() {
  const searchInput = document.getElementById('search-input');
  const searchResults = document.getElementById('search-results');

  searchInput.addEventListener('input', function() {
    const query = searchInput.value.toLowerCase();
    const results = searchDocumentation(query);
    displayResults(results);
  });

  function searchDocumentation(query) {
    const sections = document.querySelectorAll('section');
    const results = [];

    sections.forEach(section => {
      const text = section.textContent.toLowerCase();
      if (text.includes(query)) {
        results.push(section);
      }
    });

    return results;
  }

  function displayResults(results) {
    searchResults.innerHTML = '';

    results.forEach(result => {
      const resultItem = document.createElement('div');
      resultItem.classList.add('result-item');
      resultItem.textContent = result.textContent;
      searchResults.appendChild(resultItem);
    });
  }

  const darkModeToggle = document.getElementById('dark-mode-toggle');
  const highContrastToggle = document.getElementById('high-contrast-toggle');
  const reducedAnimationToggle = document.getElementById('reduced-animation-toggle');
  const largeTextToggle = document.getElementById('large-text-toggle');

  darkModeToggle.addEventListener('click', function() {
    document.body.classList.toggle('dark-mode');
  });

  highContrastToggle.addEventListener('click', function() {
    document.body.classList.toggle('high-contrast');
  });

  reducedAnimationToggle.addEventListener('click', function() {
    document.body.classList.toggle('reduced-animation');
  });

  largeTextToggle.addEventListener('click', function() {
    document.body.classList.toggle('large-text');
  });
});
