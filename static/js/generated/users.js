(() => {
    const USERS_URL = 'api/users';
    const TBODY = document.body.querySelector('tbody');
    if (TBODY === null) {
        throw new Error('incomplete html');
    }
    let pass = '';
    const render = (users) => {
        const oldTrs = document.body.querySelectorAll('tr:not(.thead)');
        oldTrs.forEach((tr) => tr.remove());
        users.forEach((item) => {
            const tr = document.createElement('tr');
            tr.innerHTML = `<td>${item.ID}</td>`
                + `<td>${item.firstname}</td>`
                + `<td>${item.phone}</td>`
                + `<td>${item.birthdate}</td>`
                + `<td>${item.created_at.split('T')[0]}</td>`
                + `<td><ul>${item.promo.map((pr) => `<li><span>${pr.promocode}${pr.activated_at === null ? '' : '<br>(активирован)'}</li>`)}</ul></td>`;
            TBODY.append(tr);
        });
    };
    const fetchUsers = () => {
        return fetch(USERS_URL, { headers: { authorization: pass } })
            .then((raw) => {
            if (raw.ok !== true) {
                throw new Error('fuck');
            }
            return raw.json();
        });
    };
    const handleAuthorized = () => {
        pass = prompt('Введите пароль') || '';
        return fetchUsers();
    };
    const handleApiData = (data) => {
        switch (data.status) {
            case 0 /* EScenarioStatus.SCENARIO_SUCCESS */:
                return render(data.payload);
            case 1 /* EScenarioStatus.UNAUTHORIZED */:
                handleAuthorized().then(handleApiData);
                return;
            case 3 /* EScenarioStatus.SYSTEM_ERROR */:
                throw new Error('Не удалось получить данные');
            default:
                throw new Error('Неизвестный статус ответа');
        }
    };
    window.addEventListener('load', () => {
        fetchUsers().then((result) => handleApiData(result));
    }, { once: true });
})();

