from flask import Flask
from flask import render_template
from flask import request

from qshifter import QuickShifter
app = Flask(__name__)


# index页面
@app.route('/')
def hello_world(name='world'):  # put application's code here
    if (request.args.get('name') is not None):
        name = request.args.get('name')
    return render_template('test.html',name=name)

# 文本输入路由
@app.route('/qshifter')
def qshifter():
    return render_template('form.html')

@app.route('/api/qshifter', methods=['POST'])
def api_qshifter():
    # TODO: 增加文件上传防护
    # 文件上传处理
    file = request.files.get('file')
    if file:
        filename = file.filename
        # 将上传的文件留档
        # file.save(f'uploads/{filename}')
        # 读取文件内容并逐行处理
        content = file.stream.read().decode('utf-8').splitlines()
        # 在这里可以对每一行进行处理
        processed_lines = [line.strip() for line in content]
        result = []
        for line in processed_lines:
            text = QuickShifter(line)
            array = text.shifts
            result.append(array)
        return render_template('result_file.html',result=result)

    # 文本输入处理
    str = request.form.get('qshift')
    if str:
        test = QuickShifter(str)
        array = test.shifts

    # 将结果传递到视图模板中，遍历输出其中的元素
    return render_template('result.html',result=array)


if __name__ == '__main__':
    app.run()
